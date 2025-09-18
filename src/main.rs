use nlopt::Nlopt;
use statrs::distribution::ContinuousCDF;

use statrs::distribution::Normal;

const M: usize = 30;

const GAMMA: f64 = 0.3;

fn obj_fn(z: &[f64], _grad: Option<&mut [f64]>, p: &mut Vec<f64>) -> f64 {
    (p.iter()
        .zip(z)
        .map(|(p_i, z_i)| p_i * z_i.powi(4))
        .sum::<f64>()
        - 3.0)
        .powi(2)
}

fn mean_constraint(z: &[f64], _grad: Option<&mut [f64]>, p: &mut Vec<f64>) -> f64 {
    p.iter().zip(z).map(|(p_i, z_i)| p_i * z_i).sum::<f64>()
}

fn var_constraint(z: &[f64], _grad: Option<&mut [f64]>, p: &mut Vec<f64>) -> f64 {
    p.iter()
        .zip(z)
        .map(|(p_i, z_i)| p_i * z_i.powi(2))
        .sum::<f64>()
        - 1.0
}

fn main() -> anyhow::Result<()> {
    let mut q = vec![0.0; M];
    for j in 1..=M / 2 {
        q[j - 1] = (j as f64 - 0.5).powf(GAMMA) / M as f64;
    }
    for j in 1..=M / 2 {
        q[M - j] = q[j - 1];
    }
    let mean = q.iter().sum::<f64>();
    let p: Vec<f64> = q.iter().map(|q_i| q_i / mean).collect();
    println!("{p:?}");

    // NLS of Z
    let mut Z = vec![0.0; M + 1];
    let normal = Normal::standard();
    for i in 1..=M {
        Z[i] = normal.inverse_cdf(p[0..=i - 1].iter().sum())
    }
    Z[0] = f64::NEG_INFINITY;
    Z[M] = f64::INFINITY;
    println!("{Z:?}");

    let mut z_init = vec![0.0; M];
    for i in 1..=M - 2 {
        z_init[i] = (Z[i] + Z[i + 1]) / 2.0;
    }
    z_init[0] = Z[1] - (Z[2] - Z[1]) / 2.0;
    z_init[M - 1] = Z[M - 2] + (Z[M - 1] - Z[M - 2]) / 2.0;

    let lower_bounds = &Z[..M];
    let upper_bounds = &Z[1..];

    println!("lower_bounds len: {}", lower_bounds.len());
    println!("upper_bounds len: {}", upper_bounds.len());

    let mut nl = Nlopt::new(
        nlopt::Algorithm::DirectL,
        M,
        obj_fn,
        nlopt::Target::Minimize,
        p.clone(),
    );
    nl.add_equality_constraint(mean_constraint, p.clone(), 1e-5)
        .unwrap();
    nl.add_equality_constraint(var_constraint, p.clone(), 1e-5)
        .unwrap();
    nl.set_lower_bounds(lower_bounds).unwrap();
    nl.set_upper_bounds(upper_bounds).unwrap();
    nl.optimize(&mut z_init).unwrap();

    println!("{z_init:?}");

    Ok(())
}
