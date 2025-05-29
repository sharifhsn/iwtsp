use nalgebra::{DMatrix, DVector};
use statrs::{
    distribution::{Continuous, ContinuousCDF, Normal},
    function::gamma,
};
use std::time::Instant;

pub struct Sampler {
    pub m: usize,
    pub gamma: f64,
    pub q: DVector<f64>,
}

impl Sampler {
    fn new(m: usize, gamma: f64) -> Self {
        let mut q = DVector::from_fn(m, |i, _| {
            // indexed at 0
            let i = if i >= m / 2 {
                // symmetry
                m - i
            } else {
                i + 1
            };
            println!("i = {i}");
            // Xu et al. (2013) p. 863 equation (6)
            (i as f64 - 0.5).powf(gamma) / m as f64
        });
        q /= q.sum();
        println!("q = {}", q);
        Self { m, gamma, q }
    }

    fn ls(&self, z: &DVector<f64>) -> f64 {
        // Xu et al. (2013) p. 863 equation (8)
        self.q.dot(&z.map(|z_i| z_i.powi(4))) - 3.0
    }
}

const LARGE_COST: f64 = 1_000_000_000_000_000.0;
const TOL: f64 = 1e-5;

impl CostFunction for Sampler {
    type Param = DVector<f64>;
    type Output = f64;
    fn cost(&self, z: &Self::Param) -> Result<Self::Output, argmin_math::Error> {
        // Xu et al. (2013) p. 863 equation (8)

        // first constraint
        if self.q.dot(z).abs() > TOL {
            println!("first constraint failed: {}", self.q.dot(z));
            return Ok(LARGE_COST);
        }
        // second constraint
        if (self.q.dot(&z.map(|z_i| z_i.powi(2))) - 1.0).abs() > TOL {
            return Ok(LARGE_COST);
        }
        // third constraint
        // Xu et al. (2013) p. 863 paragraph 2
        let Z: Vec<f64> = (0..=self.m)
            .map(|i| match i {
                0 => f64::MIN,
                // Rust doesn't allow matching against variables
                i if i == self.m => f64::MAX,
                _ => Normal::standard().inverse_cdf(self.q.iter().take(i).sum()),
            })
            .collect();
        assert!(Z.len() == self.m + 1);
        if z.iter()
            .enumerate()
            .any(|(i, z_i)| z_i - Z[i - 1] < TOL || Z[i] - z_i < TOL)
        {
            return Ok(LARGE_COST);
        }

        // least squares
        Ok((self.ls(z)).powi(2))
    }
}

impl Gradient for Sampler {
    type Param = DVector<f64>;
    type Gradient = DVector<f64>;

    fn gradient(&self, z: &Self::Param) -> Result<Self::Gradient, argmin_math::Error> {
        Ok(self
            .q
            .clone()
            .component_mul(&z.map(|z_i| 8.0 * z_i.powi(3) * self.ls(z))))
    }
}

impl Jacobian for Sampler {
    type Param = DVector<f64>;
    type Jacobian = DMatrix<f64>;

    fn jacobian(&self, z: &Self::Param) -> Result<Self::Jacobian, argmin_math::Error> {
        Ok(DMatrix::from_fn(2, self.m, |i, j| {
            if i == 0 {
                self.q[j]
            } else {
                2.0 * self.q[j] * z[j]
            }
        }))
    }
}

impl Hessian for Sampler {
    type Param = DVector<f64>;
    type Hessian = DMatrix<f64>;

    fn hessian(&self, z: &Self::Param) -> Result<Self::Hessian, argmin_math::Error> {
        Ok(DMatrix::from_fn(self.m, self.m, |i, j| {
            if i == j {
                24.0 * self.q[i] * z[i].powi(2) * self.ls(z)
                    + 32.0 * self.q[i].powi(2) * z[i].powi(6)
            } else {
                32.0 * self.q[i] * self.q[j] * z[i].powi(3) * z[j].powi(3)
            }
        }))
    }
}

fn solve(problem: Sampler, solver: NelderMead<DVector<f64>, f64>) -> DVector<f64> {
    let res = Executor::new(problem, solver).run().unwrap();
    let state = res.state();
    println!("Optimization time: {:?}", state.get_time());
    println!("Number of iterations: {}", state.get_iter());
    println!("Best cost: {}", state.get_best_cost());
    state.get_best_param().unwrap().clone()
}

use argmin::{
    core::{CostFunction, Executor, Gradient, Hessian, Jacobian, State},
    solver::neldermead::NelderMead,
};
pub struct WillowTree {
    /// The number of time steps
    pub N: usize,
    /// The number of spatial nodes at each time step
    pub m: usize,
    /// Time to end of tree
    pub T: f64,
    /// Time step size
    pub h: f64,
    /// The willow tree structure
    pub tree: DMatrix<f64>,
}

impl WillowTree {
    pub fn new(N: usize, m: usize, T: f64) -> Self {
        let h = T / N as f64;
        let tree = DMatrix::zeros(N, m + 1);
        Self { N, m, T, h, tree }
    }

    pub fn solve(&self, k: usize) -> f64 {
        let v: DVector<f64> = DVector::zeros(self.m * self.m);

        todo!()
    }
}

/// Recombining tree structure for the Willow Tree model.
pub struct EuropeanOption {
    /// Initial underlying asset price
    pub S0: f64,
    /// Risk-free interest rate
    pub r: f64,
    /// Volatility of the underlying asset
    pub sigma: f64,
    /// Time to maturity in years
    pub T: f64,
    /// Number of discrete time steps
    pub N: usize,
    /// Number of spatial nodes at each time step
    ///
    /// Accuracy improvement becomes insignificant beyond 30 nodes
    pub m: usize,
    /// Time step size
    pub dt: f64,
}

impl EuropeanOption {
    pub fn new(S0: f64, r: f64, sigma: f64, T: f64, N: usize, m: usize) -> Self {
        Self {
            S0,
            r,
            sigma,
            T,
            N,
            m,
            dt: T / N as f64,
        }
    }

    pub fn markov_process(&self) -> f64 {
        let mut discrete_normal = vec![];
        for i in 1..=self.m {
            // z is the representative normal variate
            // in other words, the value that the standard normal takes
            let z_i = Normal::standard().inverse_cdf((i as f64 - 0.5) / self.m as f64);
            // q_i is the probability of z_i occurring
            // we are dividing the standard normal equally into m intervals
            // so the probability of each interval is 1/m
            let q_i = 1.0 / self.m as f64;
            discrete_normal.push((z_i, q_i));
        }
        todo!()
    }
}

pub fn a() {
    let m = 30;

    let init_param: DVector<f64> = DVector::from_iterator(m, 1..=m)
        .map(|i| Normal::standard().inverse_cdf((i as f64 - 0.5) / m as f64));
    let simplex = vec![
        init_param.clone(),
        init_param.map(|x| x + 0.01),
        init_param.map(|x| x - 0.01),
    ];
    let solver = NelderMead::new(simplex);
    let z = solve(Sampler::new(m, 0.3), solver);
    println!("z = {}", z);
    // let num_gammas = 1000;
    // let gammas: Vec<f64> = (0..num_gammas)
    //     .map(|i| i as f64 / (num_gammas as f64 - 1.0))
    //     .collect();

    // let mut zs = Vec::with_capacity(num_gammas);
    // let start = Instant::now();
    // for &gamma in &gammas {
    //     let solver = NelderMead::new(simplex.clone());
    //     let z = solve(Sampler::new(m, gamma), solver);
    //     zs.push(z);
    // }
    // let duration = start.elapsed();
    // println!(
    //     "Benchmark for {} gamma values took {:?}",
    //     num_gammas, duration
    // );
}
