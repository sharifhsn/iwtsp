# Optimization in the Implied Willow Tree

This document provides a comprehensive overview of the optimization problems inherent in the Implied Willow Tree (IWT) method. It details how these problems are solved in the reference MATLAB implementation using `fmincon` and provides a guide for replacing this functionality with the `nlopt-rs` library in Rust.

-----

## The Optimization Problems in the IWT

Constructing an Implied Willow Tree involves solving two primary constrained nonlinear optimization problems at each time step of the tree:

1. **Node Probability Optimization (`q`)**: This involves finding the vector of probabilities, **`q`**, for each of the `m` nodes at a specific time step. The goal is to minimize the difference between the option prices observed in the market and the prices calculated using the willow tree's node values (`S`) and their probabilities (`q`). For a 30-node tree, this is a **30-variable** problem.

2. **Transition Probability Optimization (`p`)**: This involves finding the **$m \\times m$** matrix of transition probabilities, **`p`**, that describes the likelihood of moving from each node at time $t_k$ to every other node at time $t_{k+1}$. The objective is to ensure the evolution of probabilities from one time step to the next is consistent with the calculated node probabilities (`q`) at both steps. For a 30-node tree, this is a **900-variable** problem.

-----

## How `fmincon` Solves the Problems in MATLAB

The reference implementation uses MATLAB's `fmincon` function, a powerful tool for constrained nonlinear optimization.

### `fmincon` API Usage

The function is called with the following general syntax:
`[x, fval] = fmincon(fun, x0, A, b, Aeq, beq, lb, ub, nonlcon, options)`

* `fun`: The objective function to minimize.
* `x0`: An initial guess for the solution.
* `A, b`: Matrices defining linear inequality constraints (`A*x <= b`).
* `Aeq, beq`: Matrices defining linear equality constraints (`Aeq*x = beq`).
* `lb, ub`: Lower and upper bounds on the solution variables.
* `nonlcon`: A function defining nonlinear constraints.
* `options`: A struct to configure the solver, where the algorithm is explicitly set to **`'sqp'` (Sequential Quadratic Programming)**.

### Example: Solving for `q` in `ImpWT_given_moments_underQ.m`

```matlab
% Define linear equality constraints: sum(q)=1 and a risk-neutral pricing condition
Aeq = [ones(1,m); S(:,n_t)'./S0*B0(n_t)];
Beq = [1; 1];

% Define bounds: 0 <= q <= 1
LB = zeros(m,1);
UB = ones(m,1);

% Set solver options
options = optimoptions(@fmincon, 'Display','iter','Algorithm','sqp', ...);

% Call fmincon to find the optimal node probabilities
[qn_opt, fval] = fmincon(@(qq)obj_q(qq, S(:,n_t), K, ...
    call_mkt(:,n_t), put_mkt(:,n_t), B0(n_t)),...
    x0,[],[],Aeq,Beq,LB,UB,[],options);
```

-----

## Replacing `fmincon` with `Nlopt` in Rust

The `nlopt-rs` library provides the tools needed to replicate the `fmincon` functionality in Rust.

### Algorithm Selection

The direct equivalent to MATLAB's `'sqp'` algorithm in `Nlopt` is **`Algorithm::Slsqp`** (Sequential Least-Squares Programming). This makes it the ideal choice for a direct port.

### API Mapping

| `fmincon` Argument | Description | `Nlopt` Rust Method |
| :--- | :--- | :--- |
| `fun` | Handle to the objective function | `set_min_objective(&mut self, f: Func, ...)` |
| `x0` | Initial guess for the variables | The initial slice `&mut [f64]` passed to `nlopt.optimize()` |
| `A`, `b` | Linear inequality constraints (`A*x <= b`) | `add_inequality_constraint(&mut self, fc: Func, ...)` |
| `Aeq`, `beq` | Linear equality constraints (`Aeq*x = beq`) | `add_equality_constraint(&mut self, h: Func, ...)` |
| `lb`, `ub` | Lower and upper bounds on variables | `set_lower_bounds(&mut self, lb: &[f64])` and `set_upper_bounds(&mut self, ub: &[f64])` |
| `nonlcon`| Nonlinear constraints function| `add_inequality_constraint` / `add_equality_constraint`|
| `options`| Solver options (e.g., algorithm)| `Nlopt::new(Algorithm::Slsqp, ...)`|

### Rust Implementation Example

Here is a complete example of setting up and solving the `q` optimization problem in Rust.

```rust
use nlopt::*;

// Struct to pass external data to the objective function
struct ObjectiveData {
    s: Vec<f64>,
    k: Vec<f64>,
    call_mkt: Vec<f64>,
    put_mkt: Vec<f64>,
    b0: f64,
}

// Struct for the second equality constraint's data
struct Constraint2Data {
    s: Vec<f64>,
    s0: f64,
    b0: f64,
}

fn run_optimization() -> Result<(), NloptError> {
    let m = 30; // Number of nodes

    // --- Assume all necessary data structs are populated here ---
    let mut objective_data = ObjectiveData { /* ... */ };
    let mut constraint2_data = Constraint2Data { /* ... */ };

    // 1. Define the objective function (logic from obj_q.m)
    let objective_fn = |q: &[f64], _gradient: Option<&mut [f64]>, data: &mut ObjectiveData| -> f64 {
        let mut se = 0.0;
        for i in 0..data.k.len() {
            let mut call_model = 0.0;
            let mut put_model = 0.0;
            for j in 0..q.len() {
                call_model += q[j] * (data.s[j] - data.k[i]).max(0.0);
                put_model += q[j] * (data.k[i] - data.s[j]).max(0.0);
            }
            call_model *= data.b0;
            put_model *= data.b0;
            se += (data.call_mkt[i] - call_model).powi(2);
            se += (data.put_mkt[i] - put_model).powi(2);
        }
        se / (2.0 * data.k.len() as f64)
    };
    
    // 2. Define constraint functions
    let equality_constraint_1 = |q: &[f64], _, _: &mut ()| q.iter().sum::<f64>() - 1.0;
    let equality_constraint_2 = |q: &[f64], _, data: &mut Constraint2Data| -> f64 {
        q.iter().zip(data.s.iter()).map(|(&qi, &si)| qi * si / data.s0 * data.b0).sum::<f64>() - 1.0
    };

    // 3. Initialize the optimizer
    let mut optimizer = Nlopt::new(Algorithm::Slsqp, m, objective_fn, Target::Minimize, &mut objective_data);

    // 4. Set bounds and constraints
    optimizer.set_lower_bounds(&vec![0.0; m])?;
    optimizer.set_upper_bounds(&vec![1.0; m])?;
    optimizer.add_equality_constraint(equality_constraint_1, &mut (), 1e-8)?;
    optimizer.add_equality_constraint(equality_constraint_2, &mut constraint2_data, 1e-8)?;
    
    // 5. Run the optimization
    let mut q_optimal = vec![1.0 / m as f64; m]; // Initial guess
    let mut final_objective_value = 0.0;
    optimizer.optimize(&mut q_optimal, &mut final_objective_value)?;
    
    println!("Found minimum at q = {:?}", q_optimal);
    Ok(())
}
```

-----

## IPOPT vs. SQP for This Problem

For a 30-node tree, both IPOPT (an **Interior-Point** solver) and `Slsqp` (an **SQP** solver) are excellent choices, but they have different strengths.

* **SQP (active-set method):** Explores the *boundary* of the feasible region. It's very efficient for small to medium-sized problems.
* **IPOPT (barrier method):** Traverses the *interior* of the feasible region, which scales better for large problems.

| Feature | IPOPT (Interior-Point) | Nlopt's `Slsqp` (SQP) |
| :--- | :--- | :--- |
| **Scalability** | **Excellent.** Superior for large problems. This is a key advantage for the 900-variable `p` vector. | **Good.** Effective for the 30-node case, but performance can degrade more than IPOPT on larger problems. |
| **Computational Cost** | Higher per iteration (solves a larger linear system). | Lower per iteration (solves a smaller quadratic subproblem). |
| **Convergence** | Often requires fewer iterations. | May require more, but cheaper, iterations. |

**Recommendation for m=30:**

* **For `q` (30 variables):** Both are excellent. `Slsqp` might be slightly faster.
* **For `p` (900 variables):** **IPOPT** is the more robust and scalable choice due to the high number of variables.

-----

## Why Solving for `p` is a 900-Variable Problem

The transition probability `p` is an **$m \times m$** matrix where each element $p_{ij}$ is the probability of moving from node $i$ at time $t_k$ to node $j$ at time $t_{k+1}$.

For a tree with **$m=30$** nodes, this matrix is **$30 \times 30$**. To fully define it, the optimizer must solve for every single element simultaneously.

$$\text{Total Variables} = m \times m = 30 \times 30 = \textbf{900}$$

The MATLAB code confirms this by creating a vector of size `m*m` for the optimization process and then reshaping the result back into an `m x m` matrix.
