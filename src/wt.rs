use argmin::{
    core::{CostFunction, Executor, Gradient, Hessian, Jacobian, State},
    solver::{
        linesearch::{BacktrackingLineSearch, condition::ArmijoCondition},
        neldermead::NelderMead,
        quasinewton::LBFGS,
    },
};
use nalgebra::{DMatrix, DVector, dvector};
use statrs::{
    distribution::{Continuous, ContinuousCDF, Normal},
    function::gamma,
};
use std::time::Instant;

use std::sync::LazyLock;

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
            // Xu et al. (2013) p. 863 equation (6)
            (i as f64 - 0.5).powf(gamma) / m as f64
        });
        q /= q.sum();
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
        let mean = self.q.dot(z).abs();
        if mean > TOL {
            println!("first constraint failed: {}", self.q.dot(z));
            return Ok(LARGE_COST * (mean - TOL));
        }
        // second constraint
        let var = self.q.dot(&z.map(|z_i| z_i.powi(2))).abs();
        if (var - 1.0).abs() > TOL {
            println!("q = {}", self.q);
            println!("z = {}", z);
            println!(
                "second constraint failed: {}",
                self.q.dot(&z.map(|z_i| z_i.powi(2)))
            );
            return Ok(LARGE_COST * (var - 1.0 - TOL).abs());
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
        // println!("z = {:?}", z);
        assert!(Z.len() == self.m + 1);
        if z.iter()
            .enumerate()
            .any(|(i, z_i)| z_i - Z[i] < TOL || Z[i + 1] - z_i < TOL)
        {
            println!("third constraint failed: z = {:?}", z);
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

pub fn a() {
    let m = 30;
    let gamma = 0.3;

    let mut q = DVector::from_fn(m, |i, _| {
        // indexed at 0
        let i = if i >= m / 2 {
            // symmetry
            m - i
        } else {
            i + 1
        };
        // Xu et al. (2013) p. 863 equation (6)
        (i as f64 - 0.5).powf(gamma) / m as f64
    });
    q /= q.sum();

    let Z: Vec<f64> = (0..=m)
        .map(|i| match i {
            0 => -LARGE_COST,
            // Rust doesn't allow matching against variables
            i if i == m => LARGE_COST,
            _ => Normal::standard().inverse_cdf(q.iter().take(i).sum()),
        })
        .collect();
    println!("Z = {:?}", Z);

    let init_param = DVector::from_iterator(m, 1..=m).map(|i| {
        let mid = (Z[i - 1] + Z[i]) / 2.0;
        if mid > 10.0 {
            Z[i - 1] + 0.5
        } else if mid < -10.0 {
            Z[i] - 0.5
        } else {
            mid
        }
    });
    println!("real init_param = {}", init_param);
    // let mut init_param = dvector![
    //     -2.5908, -1.9534, -1.6549, -1.4302, -1.2463, -1.0877, -0.9461, -0.8165, -0.6955, -0.5809,
    //     -0.4708, -0.3637, -0.2586, -0.1541, -0.0507
    // ];
    // let binding = init_param.clone();
    // let pos = binding.into_iter().rev().map(|x: &f64| x.abs());
    // init_param.extend(pos);
    // println!("init_param = {}", init_param);
    let linesearch = BacktrackingLineSearch::new(ArmijoCondition::new(TOL).unwrap());
    let solver = LBFGS::new(linesearch, 100);
    // let z = solve(Sampler::new(m, 0.3), solver);
    // println!("z = {}", z);
    let problem = Sampler::new(m, gamma);
    let res = Executor::new(problem, solver)
        .configure(|state| state.param(init_param))
        .run()
        .unwrap();
    let state = res.state();
    println!("Optimization time: {:?}", state.get_time());
    println!("Number of iterations: {}", state.get_iter());
    println!("Best cost: {}", state.get_best_cost());
    let z = state.get_best_param().unwrap().clone();
    println!("Best parameter: {}", z);
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
