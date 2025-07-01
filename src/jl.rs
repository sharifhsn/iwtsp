use jlrs::prelude::*;

use statrs::distribution::{ContinuousCDF, Normal};

const LARGE_COST: f64 = 3.0;

pub fn sampler() {
    // number of nodes in tree
    let m = 30;

    // sampling parameter
    let gamma = 0.3;

    let mut q = (1..=m / 2)
        .map(|i| {
            // Xu et al. (2013) p. 863 equation (6)
            (i as f64 - 0.5).powf(gamma) / m as f64
        })
        .collect::<Vec<f64>>();
    q.extend_from_within(..);
    for i in 1..=m / 2 {
        // symmetry
        // Xu et al. (2013) p. 863 paragraph 1
        // modified to use 0-based indexing
        q[m - i] = q[i - 1];
    }
    // Xu et al. (2013) p. 864 equation (7)
    let sum_q: f64 = q.iter().sum();
    q = q.iter().map(|x| x / sum_q).collect();

    // calculate Z interval boundaries
    // Xu et al. (2013) p. 864 paragraph 2
    let mut Z: Vec<f64> = vec![0.0; m + 1];
    for i in 1..=m / 2 {
        Z[i] = Normal::standard().inverse_cdf(q.iter().take(i).sum());
    }
    Z[0] = -LARGE_COST;
    for i in 1..=m / 2 {
        // symmetry
        Z[m + 1 - i] = -Z[i - 1];
    }
    Z[m / 2] = 0.0;

    // set an initial guess for z based on midpoint of Z intervals
    // Xu et al. (2013) p. 864 paragraph 3
    let z = Z
        .windows(2)
        .map(|w| (w[0] + w[1]) / 2.0)
        .collect::<Vec<f64>>();

    // print all inputs
    println!("q: {:?}", q);
    println!("Z: {:?}", Z);
    println!("z: {:?}", z);

    // move into Julia context for optimization

    // initialize handle to Julia runtime
    // this will start Julia in the current thread, only initialized once!
    let handle = Builder::new().start_local().unwrap();

    // load NonlinearSolve.jl
    unsafe { handle.using("NonlinearSolve") }.expect("NonlinearSolve package failed to load");

    handle.local_scope::<_, 9>(|mut frame| {
        // acquire the FastShortcutNLLSPolyalg function
        let nlls_fn = Module::main(&frame)
            .global(&mut frame, "FastShortcutNLLSPolyalg")
            .expect("NonlinearSolve module not loaded");

        // Convert Rust data to Julia arrays
        let q_julia = TypedArray::<f64>::from_slice_copied(&mut frame, &q, q.len())
            .expect("Failed to create Julia array for q")
            .expect("Something about this array is wrong");

        let Z_julia = TypedArray::<f64>::from_slice_copied(&mut frame, &Z, Z.len())
            .expect("Failed to create Julia array for Z")
            .expect("Something about this array is wrong");

        let z_julia = TypedArray::<f64>::from_slice_copied(&mut frame, &z, z.len())
            .expect("Failed to create Julia array for z")
            .expect("Something about this array is wrong");

        //     // Define the residual function
        // let residual_fn = unsafe {
        //     Value::eval_string(
        //         &mut frame,
        //         r#"
        //         function nlls_residuals(z, p)
        //             q, Z = p
        //             m = length(q)

        //             # Main objective residual
        //             objective_residual = sum(q .* z.^4) - 3.0

        //             # Equality constraint residuals (penalized)
        //             constraint1_residual = 1e6 * sum(q .* z)         # mean = 0
        //             constraint2_residual = 1e6 * (sum(q .* z.^2) - 1.0)  # variance = 1

        //             # Box constraint penalties
        //             bound_penalty = 0.0
        //             for i in 1:m
        //                 if z[i] < Z[i] || z[i] > Z[i+1]
        //                     bound_penalty += 1e6 * (min(z[i] - Z[i+1], 0.0)^2 + min(Z[i] - z[i], 0.0)^2)
        //                 end
        //             end

        //             return [objective_residual, constraint1_residual, constraint2_residual, bound_penalty]
        //         end
        //         "#,
        //     )
        // }.expect("Failed to define residual function");

        // // Create parameter tuple
        // let params = unsafe {
        //     Value::eval_string(&mut frame, "(q_julia, Z_julia)").unwrap()
        // };

        // // Create and solve the problem
        // let problem = unsafe {
        //     Value::eval_string(&mut frame, "NonlinearLeastSquaresProblem(nlls_residuals, z_julia, params)")
        //         .unwrap()
        // };

        // let solution = unsafe {
        //     Value::eval_string(&mut frame, "solve(problem, FastShortcutNLLSPolyalg())")
        //         .unwrap()
        // };

        // // Extract results
        // let z_solution = unsafe {
        //     Value::eval_string(&mut frame, "solution.u").unwrap()
        // };

        // println!("Solution: {z_solution:?}");
    });
}
