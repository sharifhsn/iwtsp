# Implied Willow Trees for Stochastic Processes

This repository contains the implementation and research related to constructing implied willow trees for various stochastic processes, as part of a Master's thesis in Financial Engineering at Stevens Institute of Technology. The thesis is advised by Dr. Zhenyu Cui and read by Professor Dragos Bozdog. The general topic is the construction of implied willow trees for different stochastic processes, aiming to represent comprehensive market information across equity, fixed income, credit, and VIX segments under a unified framework known as the "Implied Willow Forest" (IWF).

## Project Overview

The core objective is to develop a nonparametric method to calibrate market data and create a useful structure (IWF) for pricing financial derivatives, forecasting, and trading. This involves not only growing individual implied willow trees from market-observable data but also incorporating the intricate connections between different market segments.

## Key Concepts

* **Willow Tree (WT)**: A recombining tree structure, similar to binomial or trinomial trees, but with a fixed number of nodes, typically `m = 30`. It approximates Brownian motion between nodes using a discrete Markov process.
* **Implied Willow Tree (IWT)**: An extension of the willow tree that extracts risk-neutral densities (RNDs) using market information, specifically options data. It uses the Johnson curve to estimate the RND from extracted moments. The IWT procedure involves mapping samples from a standard normal distribution to the real density using a formula where parameters are solved by the Hill and Holder algorithm.
* **Joint Implied Willow Tree (JIWT)**: An improved method for the IWT, specifically designed for pricing VIX options. It runs the IWT procedure on SPX to construct RNDs and then incorporates VIX values into the transition probabilities.
* **Implied Willow Forest (IWF)**: A proposed overarching framework that represents all market information across several segments (equity, fixed income, credit, and VIX) under one umbrella. It aims to incorporate intricate connections between these segments explicitly.
* **Markov Process**: A stochastic process where the future state depends only on the current state, not on the sequence of events that preceded it. In the context of the willow tree, an embedded Markov chain describes the transitions between states (nodes) at each time step. The probabilities of these transitions are governed by a transition probability matrix `P`.
* **Risk-Neutral Density (RND)**: A probability distribution that, when used to discount expected future payoffs, yields the current market price of an asset. The IWT aims to extract and estimate these densities.
* **Johnson Curve**: A system of frequency curves used to approximate moments and estimate RNDs, allowing a direct mapping from standard normal to real density using four moments.

## Methodology

The construction of the willow tree and its implied extensions involves solving a linear programming problem for the transition probabilities (`P`). The constraints for this optimization problem ensure properties like martingale convergence and fast convergence through a uniform stationary distribution. An improved sampling method introduced by Xu et al. (2013) is also incorporated.

### Optimization

The optimization problems are solved using numerical solvers. The current implementation utilizes the `rust-or/good_lp` library.

## Applications and Scope

The framework is being developed to address various financial instruments and markets:

* **Equity and VIX**: Utilizing JIWT for SPX and VIX options.
* **Fixed Income/Short Rate**: Constructing IWT for the Heath-Jarrow-Merton (HJM) model, using options written on the short rate or interest rate sensitive options to extract moments or compute them parametrically.
* **Credit**: Exploring the application of JIWT with equity and credit options (e.g., convertible bonds), and investigating the credit-implied volatility (CIV) derived from CDS spreads.

The ultimate goal is to build a correlated structure that links these different market segments.

## Technical Details

The project is primarily implemented in **Rust**. Key libraries used include:

* `nalgebra`: For linear algebra operations, including dense matrices and vectors.
* `statrs`: For statistical distributions, particularly the standard normal distribution and its inverse CDF.
* `argmin`: An optimization framework, specifically utilizing the `NelderMead` solver for finding optimal parameters.

### Code Structure

* `src/lib.rs`: Defines the library module.
* `src/main.rs`: Contains the main execution logic, demonstrating the use of the `wt` module.
* `src/wt.rs`: Implements the core Willow Tree and Sampler structures, including the cost function, gradient, and Hessian for the optimization problem. It also contains the `EuropeanOption` struct for model parameters.

## Future Work

Future research directions may include:

* **Machine Learning Integration**: Exploring the use of neural networks (e.g., LSTM, convolutional networks, transformers) to represent market information, conceptually similar to the willow tree's role in representation.
* **Implied Willow Forest (IWF) Development**: Further developing the framework to explicitly incorporate the intricate connections between equity, fixed income, credit, and VIX markets.
