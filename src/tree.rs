use nalgebra::{DMatrix, DVector};
use statrs::distribution::{ContinuousCDF, Normal};

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
