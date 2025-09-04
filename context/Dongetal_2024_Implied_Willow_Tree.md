# Implied Willow Tree

Bing Dong1, Wei Xu2† and Zhenyu Cui3

1 School of Finance, Shanghai University of International Business and Economics, Shanghai 201620, China
2 Department of Mathematics, Toronto Metropolitan University, Toronto, ON, Canada
3 School of Business, Steven Institute of Technology, Hoboken, NJ, USA
1 bdong@suibe.edu.cn
2 wei.xu@torontomu.ca
3 zcui6@stevens.edu

November 30, 2023

## Abstract

Reconstructing the risk-neutral density (RND) of an underlying asset has been extensively studied using market-observed European option prices. However, the literature on reconstructing the corresponding implied stochastic process remains limited. To address this challenge, we propose an innovative approach that incorporates an implied willow tree structure into market-observable options data across various maturities. Rather than solely focusing on recovering the RNDs, our method aims to reconstruct the risk-neutral process (RNP) without relying on any prior parametric models. This unique feature enables data-driven model-free valuation of potentially path-dependent options, eliminating the need for speciﬁc parametric stochastic models. Through our numerical experiments, we demonstrate the eﬀectiveness of our approach in pricing American and Asian options, as well as computing Greeks. Furthermore, we demonstrate the resilience of our approach by assessing its performance when subjected to noise introduced into the original data. To conclude, we provide empirical evidence of the eﬃcacy of the implied willow tree method using data from S&P 500 index options.

**Keywords**: Implied willow tree, risk neutral density, path-dependent options, data-driven.

∗The work of Bing Dong is supported by the National Natural Science Foundation of China (Grant Number: 72001132), “Chenguang Program” supported by Shanghai Education Development Foundation and Shanghai Municipal Education Commission (Grant Number: 20CG62), National Social Science Foundation of China (Grant Number: 23BJY228), Shanghai Philosophy and Social Science Planning Project (Grant Number: 2022ZJB011) and research funding from the Center for Modern Finance Research, Shanghai Jiao-tong University. The second author is supported by the Natural Sciences and Engineering Research Council of Canada (RGPIN-2020-04686). The third author is supported by National Science Foundation (NSF-CNS 2113906)

†Corresponding author.

## 1 Introduction

The risk-neutral density (RND) lies at the heart of modern ﬁnance, especially in derivative pricing. Under the risk-neutral measure, an European-style derivative can be priced by taking expectation of its payoﬀ function with respect to the RND1. The main challenge and developments in derivatives pricing are hence to obtain the explicit closed-form RND for various asset price models, e.g, Black and Scholes (1973); Cox et al. (1985); Kou (2002); Merton (1976), just to mention a few. Most existing methods in the literature assume a particular form of stochastic models for the underlying asset price and then calibrate the parameters from market-observed option prices, see Bates (1996); Heston (1993). This approach is subject to model-misspeciﬁcation risk. Improper modeling assumptions can lead to poor business decisions and signiﬁcant ﬁnancial losses. Thus, the nonparametric and the semi-parametric approaches have received much attentions recently. A common non-parametric approach is based on the celebrated Breeden-Litzenberger result (Breeden and Litzenberger, 1978), and recovers the RND by numerically2 approximating the second order derivatives of the call option price surface with respect to the strike price. Only the middle portion of the RND can be reconstructed from the options’ market data while the tails have to be recovered in some other ways. Shimko (1993) and Bliss and Panigirtzoglou (2004) extended the implied volatilities at the lowest and highest available strikes into the unobserved tails, however, this forces the tails to be lognormal. Figlewski (2009) proposed ﬁtting generalized extreme value (GEV) distributions to the two tails, citing the Fisher-Tippett theorem. The implied binomial tree models (Rubinstein, 1994) represent another approach to reconstructing the dynamics. The tree forces local risk-neutral dynamics to reproduce the observed market RND at option expiration based on option prices (Derman and Kani, 1994; Elyasiani et al., 2016; Tian, 2011, 2015). Figlewski (2018) provided a thorough review of the existing literature on recovering the RND from option prices. Recently, Cui et al. (2020, 2021) proposed a nonparametric approach to reconstruct the physical density function of asset prices through the empirical characteristic function. Then, the corresponding RND can be obtained from the physical density function by enforcing the martingale property constraint. Diﬀerent from the RND recovered in Figlewski (2018), Cui et al. (2020, 2021) work with the historical asset prices, rather than the option prices, thus the reproduced RND is ex-post, not ex-ante.

There is a common limitation of all the methods mentioned above, which is that they reconstruct the RND of the underlying asset only at a particular ﬁxed maturity, rather than the full risk-neutral process of the underlying assets at several time points across the life span of available options. In their study, Jackwerth and Rubinstein (2012) employ options prices to deduce information about the risk-neutral stochastic process that governs the underlying index. They achieve this by comparing diﬀerent stochastic models. To the best of authors’ knowledge, there is currently no established methodology that primarily focuses on reconstructing the Risk-Neutral Process (RNP) without making assumptions about speciﬁc models, in contrast to the approach that focuses on the Risk-Neutral Density (RND). We resolve this challenge by embedding an implied willow tree structure for modeling the risky asset into market-observable options data across maturities.

The approach that closely relates to our work is the classical implied binomial tree method (Rubinstein, 1994). The primary distinction between our implied willow tree (IWT) and the implied binomial tree lies in the tree structure itself. Firstly, the willow tree incorporates higher-order moments derived from options data, while the binomial tree utilizes only the ﬁrst two moments. Accounting for skewness and kurtosis (i.e. third and fourth moments) in constructing the implied willow tree better accounts for the tail risk. Secondly, the willow tree maintains a ﬁxed number of nodes at each time point, except for the initial time, whereas the nodes in the binomial tree grow at a rate of O(n), where n represents the number of time points. Lastly, while the binomial tree method employs individual tree structures for each maturity to estimate the Risk-Neutral Density (RND), it lacks the capability to investigate the transition probabilities between diﬀerent maturities. In contrast, the IWT method constructs a Risk-Neutral Process (RNP) encompassing the initial asset price, potential asset prices at diﬀerent maturities, and transition probability matrices across these maturities. This uniﬁed approach captures the entire spectrum of option prices with various maturities and strike prices in a single IWT. In essence, it enables the modeling and ﬁtting of the complete term structure of European options prices without requiring the use of multiple binomial trees speciﬁc to various maturities.

There are four main contributions in this paper. First, instead of reconstructing the traditional RNDs, we aim to reconstruct the RNP of the underlying asset within the willow tree structure; second, the implied willow tree is a clear and natural improvement of the implied binomial tree (Rubinstein, 1994) since the IWT can reconstruct a RNP encompassing all maturities of options; third, the IWT adds a new model-free capability to the existing willow tree framework since it has the same structure as the existing model-based willow tree method (Curran, 2001; Xu et al., 2013). Thus, all existing exotic option pricing and risk management schemes are applicable within our proposed framework, therefore it opens the door to data-driven valuation of possibly path-dependent options without referring to any particular parametric stochastic model; fourth, our approach refrains from making any assumptions regarding the market or asset price itself, such as assuming a complete market, space-homogeneity, utility criterion, or Markovian3 behavior of prices. It is widely recognized that in the absence of market completeness but without arbitrage opportunities, there exist inﬁnitely many equivalent martingale measures. In this paper, we just present a methodology that aims to identify a suitable martingale measure capable of aligning with European option prices across various maturities. Given the absence of further assumptions or additional information, this measure represents the “best” ﬁt we can attain. Further analysis of real data through IWT is a priority for our future research endeavors.

The remainder of the paper is structured as follows. In Section 2, we propose a data-driven implied willow tree method to reconstruct the stochastic process of the underlying asset under the Q measure from market-observable option prices. Then in Section 3, numerical experiments on synthetic data generated from the geometric Brownian motion (Black and Scholes, 1973) and Merton’s jump diﬀusion model (Merton, 1976) and Heston’s stochastic volatility model (Heston, 1993) are presented. Section 4 considers an empirical example and illustrates the empirical performance of the implied willow tree method. Finally, we conclude the paper in Section 5.

## 2 Reconstruction of RNP

In this section, we present the reconstruction of the RNP of the underlying asset from market-observable option prices under a willow tree structure. The willow tree method was proposed by Curran (2001) and modiﬁed by Haussmann and Yan (2005) for vanilla option pricing under the geometric Brownian motion. Xu et al. (2013) improve the method and propose a uniﬁed structure for various stochastic models, such as geometric Brownian motion (Xu et al., 2013), jump diﬀusion models (Xu and Yin, 2014), CEV model (Dong et al., 2019), L´evy processes (Ma et al., 2021) and stochastic volatility models (Ma et al., 2020). An example of the structure is illustrated in Figure 1, which is constructed with 4 discrete time points and 5 possible asset values at each discrete time point. In a typical willow tree structure, the count of asset price nodes remains constant at each time step, typically ranging from 30 to 100 nodes. Thus, the total number of nodes is linear to the number of discrete time steps on the tree structure. The merits of the willow tree structure are its eﬃciency in storage and computation, and ﬂexibility in derivatives pricing and risk management. First, the willow tree is always recombining for various popular stochastic models in ﬁnance. The constant number of tree nodes at each discrete time (except at the initial time) makes its total number of tree nodes linear to the number of discrete time points. Second, its multi-numbered nodes at each discrete time provide a good approximation of the asset price distribution beyond the normal or lognormal distributions. The transition probabilities uncover the dynamics of the process over time. Thus, the willow tree is capable of calculating many complicated path-dependent derivative prices and also the corresponding risk measures, such as Asian option (Lu et al., 2017a), American moving average barrier option (Lu et al., 2017b), variance derivatives (Ma et al., 2021) and VaR and CVaR calculation for variable annuities (Dong et al., 2020), etc. Third, due to the uniﬁed structure of the tree for various stochastic processes, a pricing scheme for a particular stochastic model can be adapted to many other stochastic processes. Thus, it minimizes the cost of implementation and maintenance for new derivative products. In this section, we will take the tree structure illustrated in Figure 1, and reconstruct the stochastic process of the underlying asset under the Q measure from the option prices.

**Figure 1**: Graphical illustration of the willow tree lattice with 5 possible asset prices and 4 discrete time points (except the initial time).

### 2.1 Extracting the RND at individual maturities

At time $t_0$, given the European call and put option prices, $C(T_n, K)$ and $P (T_n, K)$, with various strikes K’s and maturities $\{T_n\}$, $n = 1, 2, \ldots, N$ , the stochastic process of the underlying asset can be recovered in the form of the willow tree structure in two steps: ﬁrst, extraction of the discrete RND for each maturity $T_n$; second, the determination of transition probabilities between RNDs across time points, i.e. the forward RNDs.

The RND of the log return of underlying asset, $X_{T_n} := \ln(S_{T_n}/S_0)$, at one of the maturities $T_n$ is in a discrete form of $\{(X^n_i , q^n_i )\}$, where $X^n_i$ is the $i$-th possible value of $X_{T_n} at $T_n$ and $q^n_i$ is the corresponding probability, for $i = 1, 2, \ldots, m$. Although the distribution of $X_{T_n}$ is unknown, its moments can be estimated based on the market-observable European call and put option prices with various strike prices, $K$, at $T_n$ by the following Theorem 2.1.

**Theorem 2.1** Assume that the option prices do not admit arbitrage. Given the mid-quotes of the bid and ask prices for each out-of-the-money call/put option, $Q_0(T, Kj)$, with some strikes $K_j$ ($K_j > S_0$ for call; $K_j < S_0$ for put) at maturity $T$ underlying asset price $S_T$ , the moments of the log return, $X_T := \ln \frac{S_T}{S0}$ , can be calculated as

$$\mathbb{E}_0 [X_T ] = \frac{1}{B_0(T )} − 1 + \frac{1}{B_0(T )} \sum_j \frac{\Delta K_j}{(K_j )^2} Q_0(T, K_j),$$

$$ \mathbb{E}_0 [(X_T )^k] = \frac{1}{B_0(T )} \sum_j \frac{\Delta K_j}{(K_j )^2} \left[k(k - 1) (\ln K_j S_0 )^{k-2} - k (\ln \frac{K_j}{S_0} )^{k−1} \right] Q_0(T, K_j), k \geq 2$$
(2.1)

where $B_0(T )$ is the discount factor at $T$ .

**Proof**. See Appendix A.

With the moments evaluated by Theorem 2.1, the discrete values $X^n_i$ can be determined through the Johnson curve (Johnson, 1949) by matching the first four moments of $X_{T_n}$. The Johnson curve provides a one-to-one mapping from the standard normal distribution to a desired distribution given the ﬁrst four moments. Thus, the value of X ni can be sampled as

$$X^n_i = \epsilon g^{-1} ( \tilde{z}_i − \bar{w} ) + \nu,$$

(2.2)

where $\tilde{z}_i$ is a sample value from the standard normal distribution and the parameters $\bar{w}$, $\nu$ and $\epsilon$ can be determined by the algorithm proposed in Hill and Holder (1976), and the function $g^{-1}(u)$ is deﬁned by

$$g^{-1}(u) := \begin{cases} e^u & \text{for the lognormal family,} \\ \frac{e^u - e^{-u}}{2} & \text{for the unbounded family,} \\ \frac{1}{1 + e^{-u}} & \text{for the bounded family,} \\ u & \text{for the normal family.} \end{cases}$$

Appendix B introduces an approach to generate a small number of samples ˜zi from the standard normal distribution.

We pick the Johnson curve for two reasons: ﬁrst, it matches exactly the ﬁrst four moments of the distribution at discrete times, and this allows us to take into account the skewness and kurtosis of the distribution; second, it is more eﬃcient in selecting points from the Johnson curve as compared to other family of points selection methods. It provides an accurate discrete approximation of a continuous distribution with just a few discrete points. Thus the Johnson curve prevails as the choice in the literature on designing the willow tree with applications to derivatives pricing.

Then, the corresponding probabilities $\{q^n_i \}$ for the RND of $X_{T_n}$ is the solution of the following constrained nonlinear least squares optimization problem:

$$\min_{q^n_i} \sum_K \left[(V_c(T_n, K) − V^{mkt}_c (T_n, K))^2 + (V_p(T_n, K) − V^{mkt}_p (T_n, K))^2\right]$$

s.t.
$$\sum^m_{i=1} q^n_i S^n_i = S_0 B_0(T )$$

$$\sum^m_{i=1} q^n_i = 1$$

$$q^n_i \geq 0, \text{ for }i = 1, 2, \ldots, m, $$

(2.4)

where $S^n_i = S_0 \exp(X^n_i )$. Here $V^{mkt}_c (K, T_n)$ and $V^{mkt}_p (K, T_n)$ are respectively the market prices of the European call and put options at maturity $T_n$ with strike price $K$. $V_c(T_n, K)$ and $V_p(T_n, K)$ are the computed European call and put option prices through the discrete RND $\{(X^n_i , q^n_i )\}$, which are given by

$$V_c(T_n, K) = B_0(T_n) \sum^m_{i=1} q^n_i (S^n_i − K)_+$$
and
$$V_p(T_n, K) = B_0(T_n) \sum^m_{i=1} q^n_i (K − S^n_i )_+.$$

The first constraint in (2.4) represents that the discounted asset price process is a martingale under the $\mathbb{Q}$ measure, whereas the last two constraints mean that the probabilities $\{q^n_i \}$’s sum to one and are nonnegative. After solving (2.4), a discrete RND for the log return at $T_n$, $\{(X^n_i , q^n_i )\}$, is obtained. The next step is to determine the transition probabilities, $p^n_{ij}$, between $\{X^n_i \}$ at $T_n$ and $\{X^{n+1}_j \}$ at $T_{n+1}$ given two discrete RNDs, $\{(X^ni , qni )} and {(X n+1 j , qn+1 j )}.

### 2.2 Determining the transition probabilities across maturities

In order to extract the term structure of the risk-neutral dynamics, we next describe the method to extract the transition probabilities of the implied willow tree. The transition probabilities [pnij] for the RND of XTn is the solution of the following constrained nonlinear least squares problem:

min pnij (cid:80)m j=1 ( (cid:80)m i=1 qni pnij − qn+1 j )2 (2.6)

s.t. (cid:80)m j=1 pnij = 1, (cid:80)m j=1 pnij ≥ 0, (cid:80)m j=1 pnijeX n+1 j = B0(Tn) B0(Tn+1) eX ni , for i, j = 1, 2, ..., m.

The constraints of the above optimization problem correspond to the following conditions under the Q measure, and here the transition probability matrix from X ni to X n+1 j is denoted as Pn = [pnij]:

1. Given the log return X ni at Tn, the log return at Tn+1 must take one of the possible values of X n+1 j , j = 1, 2, · · · , m, i.e., (cid:88)m j=1 pnij = 1, i = 1, 2, · · · , m.
2. No matter what the log return is at Tn, the log return at Tn+1 must evolve according to the discrete risk-neutral distribution {(X n+1 j , qn+1 j )}, i.e., (cid:88)m i=1 qni pnij = qn+1 j , j = 1, 2, · · · , m.
3. Given the asset price S0eX ni at Tn, the asset price S0eX n+1 j (subject to discounting) at Tn+1 is a martingale under Q, i.e., (cid:88)m j=1 pnijeX n+1 j = B0(Tn) B0(Tn+1) eX ni .

Therefore, the data-driven implied willow tree is formed with the tree nodes {Sni } and the transition probability matrices q = [qni ] (from 0 to Tn) and Pn = [pnij] (from Tn to Tn+1) for i, j = 1, 2, · · · , m, n = 1, 2, · · · , N − 1. With the willow tree structure, the exotic option prices can then be determined by backward induction calculations.

It is important to acknowledge that, without additional assumptions or supplementary datasets, determining the transition probability matrix in (2.6) uniquely may not be feasible. If further data is available, and in particular path-dependent option prices, then it could potentially lead to a more accurate determination of the transition probability matrices, bringing them closer to their real-world counterparts. However, it is worth noting that these types of data from the market may not be as readily accessible as European option prices or may not be traded with suﬃcient liquidity. As part of our future work, we plan to explore alternative sources of option prices data and incorporate them into the construction of the IWT.

## 3 Experiments on Synthetic Data

In this section, we apply our method to synthetic data and examine the performance of the proposed method under the Q measure. First, we generate the data from the geometric Brownian motion (GBM) (Black and Scholes, 1973), Merton’s jump diﬀusion model (MJD) (Merton, 1976) and Heston’s stochastic volatility model (HSV) (Heston, 1993), under which European option prices can be determined via the well-known analytic closed-form formulas. Then, our data-driven method is applied to recover the implied stochastic process for the underlying asset under the Q measure. Since there are bid-ask spreads on the option prices in the market, we also verify the robustness of our method on the data with noises.4 The GBM, MJD and HSV models assume that the underlying asset ST follows:

*   **Geometric Brownian motion**: the underlying asset price St under the Q measure are governed by the stochastic diﬀerential equation (SDE)
    dSt = rStdt + σdWt,
    where r is the constant risk free interest rate, σ is the volatility and Wt is Brownian motion under Q.
*   **Merton’s jump diﬀusion model**: the underlying asset St follows the SDE as
    dSt St = (r − λ¯k)dt + σdWt + [Yt − 1] dNt,
    where r is the constant risk free interest rate, Wt is the standard Q-Brownian motion, ¯k = E[Yt − 1], ln Yt ∼ N (αJ , σ2 J ), and Nt follows the Poisson process with constant intensity λ.
*   **Heston’s stochastic volatility model**: the underlying asset St is assumed to follow the stochastic process as
    (cid:26) dSt = rStdt + √ vtStdW 1 t , dvt = η(θ − vt)dt + σv √ vtdW 2 t ,
    where EQ[dW 1 t dW 2 t ] = ρdt, r is the constant risk free interest rate, η and θ are constant.

**Table 1**: Parameters for geometric Brownian motion (GBM), Merton’s jump diﬀusion (MJD), and Heston’s stochastic volatility (HSV) models.

(a) Parameters of geometric Brownian motion.
| σ | r |
| :--- | :--- |
| 0.3 | 0.05 |

(b) Parameters of Merton’s jump diﬀusion model are from Bacinello et al. (2016) which are calibrated from the S&P 500 index options on 31 December 2012.
| σ | αJ | σJ | λ | r |
| :--- | :--- | :--- | :--- | :--- |
| 0.1114 | -0.1825 | 0.1094 | 0.5282 | 0.05 |

(c) Parameters of Heston’s stochastic volatility models. Set 1 is calibrated from S&P 500 option prices from June 1988 to May 1991 (Bakshi et al., 1997); Set 2 is calibrated from daily closing values of the Dow-Jones industrial index for the period of 20 years from 1 January 1982 to 31 December 2001 (Drˇagulescu and Yakovenko, 2002). The Set 2 is the default parameters for HSV model in our experiments unless otherwise stated.
| | Set 1 (Bakshi et al., 1997) | Set 2 (Drˇagulescu and Yakovenko, 2002) | Set 3 (Lo et al., 2017) |
| :--- | :--- | :--- | :--- |
| η | 1.15 | 5 | 11.35 |
| v0 | 0.0348 | 0.0625 | 0.04 |
| r | 0.04 | 0.04 | 0.04 |
| θ | 0.0348 | 0.16 | 0.022 |
| σv | 0.39 | 0.9 | 0.618 |
| ρ | -0.64 | 0.1 | -0.64 |

**Simulation setup**:

The values of the model parameters are listed in Table 1. Unless otherwise stated, the simulation setup is as follows. For the HSV model, the Set 2 in Table 1(c) is the default parameters in our numerical experiments. The initial value of the asset is S0 = 100, and 11 strike prices among the interval of [50, 150] are considered at each maturity. The European call/put option prices under various maturities are calculated via the well-known analytic formulas for the three models on various strike prices.

We ﬁrst examine the impact of the number of the tree nodes, m, at each discrete time point on recovering the ﬁrst four moments of the underlying asset price process from the synthetic data set. Table 2 lists the ﬁrst four recovered moments of the asset price dynamics at diﬀerent maturities with various number of tree nodes. Since the dynamics of the synthetic data set are known, the exact moments are listed in the last column of the tables as benchmarks. It shows that increasing the number of nodes can improve the accuracy of the estimated ﬁrst four moments of the implied stochastic process from the options data, but the successive improvement becomes insigniﬁcant when there are more than 30 nodes at each discrete time point. Thus, to balance the tradeoﬀ between accuracy and eﬃciency, we choose m = 30 for the remaining experiments.

Next, we examine whether the implied stochastic process is sensitive to the number of strikes used in the reconstruction process. Generally, a large number of strikes in the data provide an accurate approach to recover the moments via the implied willow tree. However, the number of strikes for the options actually traded in the market is quite limited. Thus, we will examine the performance of our method under various number of strikes.

Table 3 records the computed ﬁrst four moments with various number of strikes. The initial value of the asset is S0 = 100, so the strikes are selected equally-spaced centered at S0. For example, when NK = 7, we only choose the option prices with strikes K = 70, 80, 90, 100, 110, 120 and 130 to build up the implied willow tree. When NK ≥ 40, we select strikes equally spaced in the interval [0, 400].

Table 3 shows that the accuracy of the computed ﬁrst four moments increases as a large number of strikes are considered, which is in line with the theoretical intuition in (2.1). However, the accuracy is well acceptable with a limited number of strikes. For example, when NK = 11, the computed mean, variance, skewness are good approximations of the exact moments, although the relative error between the computed kurtosis and the exact one is about 10%. Later, we will show this scale of error makes small impacts on pricing options and computing corresponding Greeks. In summary, our proposed method is not sensitive to the number of available strike prices, which implies that our implied willow tree method works well even with a limited number of strikes, as commonly seen in real market options data.

Table 4 records the computed four moments for various volatilities, jump sizes and sets of parameters for the three models in Table 1. It illustrates that our proposed method can recover the mean, variance and skewness of the dynamics accurately. Figures 2, 3 and 4 compare the computed European, American, Asian option prices and computed ∆ and Γ for European put options based on the implied stochastic process with the benchmarks.

The computed option prices using the implied willow tree method closely align with the benchmark prices. This indicates that the implied willow tree method accurately captures the implied stochastic process from the options data. Although the relative error in computed kurtosis may be around 10%, it does not signiﬁcantly impact the accuracy of option pricing. Furthermore, we compare the computed Greeks, namely ∆ and Γ, of European put options with the benchmark values. These Greeks are obtained by solving a linear least squares problem, as described in (C.18). The benchmark values are derived from closed-form formulas in the Black-Scholes model, while ﬁnite diﬀerencing is used to compute benchmarks for Merton’s jump diﬀusion model and Heston’s stochastic volatility model.

The results indicate that the computed ∆ values match the benchmarks across all models. For Γ, the computed values closely approximate the benchmarks under the Geometric Brownian Motion and Heston’s Stochastic Volatility models. However, there is a discrepancy from the benchmark for in-the-money options under Merton’s jump diﬀusion model. There are two explanations for this discrepancy. First, the ﬁnite diﬀerence method introduces signiﬁcant ﬂuctuations in the computed second-order derivative. Second, the discrete jumps in the strike price reduce the smoothness of the option price, making it challenging to accurately estimate the second-order derivative, Γ, using the ﬁnite diﬀerence method.

To assess the robustness of our method, we introduce noise to the synthetic option data in our experiments. We adopt the noise assumption outlined in Ait-Sahalia and Duarte (2003), which provides a methodology for incorporating noise that takes into account the option’s moneyness and closely resembles the microstructure observed in real option data. In this approach, a bid-ask spread, calibrated to the market data, is set at 5% of the option’s ask price. The noise distribution around the theoretical price is then modeled as uniform distribution, ranging from 0 to half of the bid-ask spread value, adjusted by a liquidity factor given by 1 + (2/0.2)|K/S0 − 1|, where K represents the strike price and S0 denotes the initial asset price.

Table 5 presents the computed ﬁrst four moments given the European option prices, both with and without noise, spanning maturities of 3, 6, and 12 months. It is evident that noise has a more pronounced impact on kurtosis compared to other moments such as mean, variance, and skewness. Figures 5, 6, and 7 depict the option prices computed using the implied willow trees based on the perturbed data. These ﬁgures demonstrate the resilience of our method; even with perturbed data, the implied willow tree successfully extracts the implied stochastic process and accurately prices the options. Notably, while the noise signiﬁcantly aﬀects moments with orders higher than two, our data-driven method remains robust in pricing options.

## 4 Empirical Experiments

In this section, the daily trading prices of S&P 500 European call and put options from January 1, 2006 to December 31, 2019 are considered to recover the dynamics of stochastic processes. Following Bondarenko and Muravyev (2023), we study the average of the best bid price (NBB) and best ask price (NBO) of the S&P 500 index option prices at 15:45, instead of the closing price.5 US Daily Treasury yield curve rates are used for the risk-free interest rates.6 Following Bardgett et al. (2019), we ﬁlter the options data as follows: we only consider options with maturity between one week and one year, and delete options whose trading volumes are zero. The mid-price of the option is set as the observation price. Table 6 provides a statistical description of the data set. The total number of S&P 500 European options traded in the market is 4, 603, 929. The daily average number of the S&P 500 call options is 495 whereas the average of the put options is 812. In order to eliminate the beginning-of week and end-of week eﬀects, we follow Lin et al. (2019) using weekly (Wednesday) closing price for our experiments. The log trading volume is adopted as the weight for the liquidity of option with respect to the strike price and maturity. We incorporate pricing errors into our optimization criterion, weighting them based on the logarithm of the trading volume of options. This approach enhances our ability to discern nuanced insights into market sentiment and dynamics, allowing us to gain a clearer understanding of the collective market outlook on the underlying asset. By assigning greater weight to options with higher trading volumes, our analysis prioritizes those that are more readily traded. High trading volumes can also signify more eﬃcient price discovery, as increased participation tends to result in option prices that more accurately reﬂect available information. Additionally, it is important to note that trading volume exhibits a famous fat-tailed distribution, which is why we employ the logarithm of the trading volume as the weighting factor. Thus, instead of solving the probability qni as in (2.4), we solve the following weighted least squares problem,

min qni (cid:80) K [ wc(Tn,K) (cid:101)w(Tn) (Vc(Tn, K) − V mkt c (Tn, K))2 + wp(Tn,K) (cid:101)w(Tn) (Vp(Tn, K) − V mkt p (Tn, K))2]

s.t. (cid:80)m i=1 qni Sni = S0 B0(T ) , (cid:80)m i=1 qni = 1, qni ≥ 0, for i = 1, 2, ..., m, (4.7)

where wc(Tn, K) and wp(Tn, K) are log trading volumes of call and put options with strike price K at maturity Tn, respectively, and (cid:101)w(Tn) is the sum of log trading volume of European options matured at Tn, i.e., (cid:101)w(Tn) = (cid:80) K (wc(Tn, K) + wp(Tn, K)).

We conducted a comparison between computed S&P 500 index option prices using the implied willow tree method (V ) and the market call/put option prices (V mkt) based on weekly data. Table 7 presents the mean relative error (MRE) and cent error of the computed option prices. The mean relative error (MRE) of the computed option prices is deﬁned as MRE := 1 N (cid:80) | V −V mkt V mkt |. The cent error is deﬁned as, center error := (cid:113) 1 N (cid:80)(V − V mkt)2 in cents, which is similar to the deﬁnition in Jackwerth and Rubinstein (2012). Our analysis covers the entire time period from 2006 to 2019, as well as three subperiods (2006-2010, 2011-2014, 2015-2019), considering various moneyness and maturities.

Overall, the mean relative error is 5.1% for call options and 2.42% for put options over the entire time period. For out-of-the-money options, the calculated relative error is relatively larger, but the corresponding cent error is relatively smaller compared to the average call and put option prices, which are 57.96 and 25.77 dollars, respectively. Table 7 also reveals that the option pricing error increases with longer maturity dates.

Finally, we incorporate the smoothness of the Risk-Neutral Density (RND) into the implied willow tree construction. We introduce an additional regularization mechanism in the estimation of probabilities, similar to the approach presented in Tian (2015). Rather than solving for the probability qni as in (4.7), we address the following weighted least squares problem:

min qni α (cid:80)m−1 i=2 (qni−1 − 2qni + qni+1)2 + (cid:80) K [ wc(Tn,K) (cid:101)w(Tn) (Vc(Tn, K) − V mkt c (Tn, K))2 + wp(Tn,K) (cid:101)w(Tn) (Vp(Tn, K) − V mkt p (Tn, K))2]

s.t. (cid:80)m i=1 qni Sni = S0 B0(T ) , (cid:80)m i=1 qni = 1, qni ≥ 0, for i = 1, 2, ..., m, (4.8)

where α is a positive smoothness parameter.

To determine the optimal value of α in (4.8), we analyze the contributions of the density smoothness term α (cid:80)m−1 i=2 (qni−1 − 2qni + qni+1)2 and the Mean Squared Error (MSE) term within the objective function presented in (4.8). The choice of α aims to strike a balance between these two terms. When α is small, for instance, α = 0.1 ∗ S0, the fourth column in Table 8 demonstrates that the MSE term dominates the objective function, resulting in irregular density patterns. As α increases, the density becomes smoother, but the MSE also increases.

In this study, we opt for α = 100S0 as it achieves a favorable equilibrium between the smoothness term and the MSE term. As shown in the seventh column of Table 8, both terms are of the same order as the selected α. In other words, both factors contribute similarly to the determination of the optimal density qni .

Figure 8 illustrates the probability densities of returns (ST /S0) between the 5% and 95% percentiles, computed using our proposed method under the risk-neutral measure on May 15, 2019, for one-week, two-week, and three-week horizons. As previously observed in Figlewski (2009), the option prices available in the market struggle to capture the tail information of the distribution. Therefore, one of our future research directions involves exploring methods to enhance the recovery of tail information.

Figure 9 displays the IWT with 30 nodes for discrete maturities of the S&P 500 index based on option data from May 15, 2019, extending up to two weeks. It is worth noting that constructing the IWT for longer maturities follows a similar methodology. This analysis commences with the value of the S&P 500 index on May 15, 2019, S0 = 2854.

Within this framework, we have two sets of values: [S1i ] and [S2j ] represent 30 possible index values for one and two weeks, respectively. The vector [q1i ] corresponds to the associated probabilities for each value in [S1i ]. These pairs, expressed as (S1i , q1i ), constitute the RND of St over one week.

Furthermore, the matrix [p1ij] characterizes the transition probabilities from [S1i ] to [S2j ]. For instance, if we consider the index value in the ﬁrst week as S19 = 2826, the 9th row of [p1ij] illustrates the transition probabilities from S19 to S2j , where j = 1, 2, · · · , 30. Speciﬁcally, p19j represents the transition probability from S19 to S2j .

In summary, the elements S0, [q1i ], [p1ij], and [S2j ] together form the RNP of St over one week and two-week horizons. The pairs (S1i , q1i ) represent the discrete RND of St over one week. The matrix [p1ij] delineates the transition probabilities from [S1i ] to [S2j ]. Additionally, introducing [q2j ] = [q1i ] · [p1ij], which denotes the vector-matrix product of [q1i ] and [p1ij], allows us to derive the pairs (S2j , q2j ) which represent the RND of St over two weeks.

## 5 Conclusion

In this paper, we propose an implied willow tree method under the Q measure based on information of the European option prices. Instead of extracting RNDs at each maturity separately, the implied willow tree structure explores the RNP of the asset prices, so it can be used for nonparametric path-dependent derivative pricing and hedging. Since the implied willow tree structure is fully data-driven and model-free under the Q measure and shares the same structure as the willow tree for the model-based approach, all existing methods for path-dependent derivative pricing and risk measures calculations (e.g. VaR and CVaR) can be similarly carried out under this data-driven approach. Through our numerical experiments, we demonstrate the eﬀectiveness of our approach in pricing American and Asian options, as well as computing Greeks (as depicted in Figures 2, 3, and 4). Moreover, we showcase the robustness of our method by evaluating its performance when noises are incorporated in the option data (as illustrated in Figures 5, 6, and 7). Finally, we illustrate the empirical performance of the implied willow tree method based on the S&P 500 index options data.

## References

Ait-Sahalia, Y. and Duarte, J. (2003). Nonparametric option pricing under shape restrictions. Journal of Econometrics 116 9–47.
Andricopoulos, A. D., Widdicks, M., Duck, P. W. and Newton, D. P. (2003). Universal option valuation using quadrature methods. Journal of Financial Economics 67 447–471.
Andricopoulos, A. D., Widdicks, M., Newton, D. P. and Duck, P. W. (2007). Extending quadrature methods to value multi-asset and complex path dependent options. Journal of Financial Economics 83 471–499.
Bacinello, A. R., Millossovich, P. and Montealegre, A. (2016). The valuation of GMWB variable annuities under alternative fund distributions and policyholder behaviours. Scandinavian Actuarial Journal 2016 446–465.
Bakshi, G., Cao, C. and Chen, Z. (1997). Empirical performance of alternative option pricing models. The Journal of Finance 52 2003–2049.
Bardgett, C., Gourier, E. and Leippold, M. (2019). Inferring volatility dynamics and risk premia from the S&P 500 and VIX markets. Journal of Financial Economics 131 593–618.
Bates, D. S. (1996). Jumps and stochastic volatility: Exchange rate processes implicit in Deutsche mark options. The Review of Financial Studies 9 69–107.
Black, F. and Scholes, M. (1973). The pricing of options and corporate liabilities. Journal of Political Economy 81 637–654.
Bliss, R. and Panigirtzoglou, N. (2004). Option-implied risk aversion estimates. Journal of Finance 59 407–446.
Bondarenko, O. and Muravyev, D. (2023). Market return around the clock: A puzzle. Journal of Financial and Quantitative Analysis 58 939–967.
Breeden, D. T. and Litzenberger, R. H. (1978). Prices of state-contingent claims implicit in option prices. Journal of Business 621–651.
Carr, P. and Madan, D. (1998). Towards a theory of volatility trading. Volatility: New estimation techniques for pricing derivatives 29 417–427.
Carr, P. and Madan, D. (2001). Optimal positioning in derivative securities. Quantitative Finance 1 19–37.
Chen, D., H¨ark¨onen, H. J. and Newton, D. P. (2014). Advancing the universality of quadrature methods to any underlying process for option pricing. Journal of Financial Economics 114 600–612.
Cox, J. C., Ingersoll Jr, J. E. and Ross, S. A. (1985). An intertemporal general equilibrium model of asset prices. Econometrica: Journal of the Econometric Society 363–384.
Cui, Z., Kirkby, J. and Nguyen, D. (2020). Nonparametric density estimation by B-spline duality. Econometric Theory 36 250–291.
Cui, Z., Kirkby, J. and Nguyen, D. (2021). A data-driven framework for consistent ﬁnancial valuation and risk measurement. European Journal of Operational Research 289 381–398.
Cui, Z. and Xu, Y. (2022). A new representation of the risk-neutral distribution and its applications. Quantitative Finance 22 817–834.
Cui, Z. and Yu, Z. (2021). A model-free Fourier cosine method for estimating the risk-neutral density. The Journal of Derivatives 29 149–171.
Curran, M. (2001). Willow power: optimizing derivative pricing trees. Algo Research Quarterly 4 15–24.
Derman, E. and Kani, I. (1994). Riding on a smile. Risk 7 32–39.
Dong, B., Jindong, W. and Xu, W. (2020). Risk metrics evaluation for variable annuity with various guaranteed beneﬁts. Journal of Derivatives 28 59–79.
Dong, B., Xu, W. and Kwok, Y. K. (2019). Willow tree algorithms for pricing guaranteed minimum withdrawal beneﬁts under jump-diﬀusion and CEV models. Quantitative Finance 19 1741–1761.
Drˇagulescu, A. A. and Yakovenko, V. M. (2002). Probability distribution of returns in the Heston model with stochastic volatility. Quantitative Finance 2 443–453.
Elyasiani, E., Muzzioli, S. and Ruggieri, A. (2016). Forecasting and pricing powers of option-implied tree models: Tranquil and volatile market conditions . URL http://hdl.handle.net/11380/1122975
Figlewski, S. (2009). Estimating the implied risk neutral density. In Volatility and Time Series Econometrics: Essays in Honor of Robert F. Engle. Oxford University Press.
Figlewski, S. (2018). Risk-neutral densities: A review. Annual Review of Financial Economics 10 329–359.
Haussmann, U. and Yan, L. (2005). The modiﬁed willow tree algorithm. Journal of Computational Finance 8 63–79.
Heston, S. L. (1993). A closed-form solution for options with stochastic volatility with applications to bond and currency options. The Review of Financial Studies 6 327–343.
Hill, I. and Holder, R. (1976). Algorithm as 99: Fitting Johnson Curves by moments. Applied Statistics 25 180–189.
Jackwerth, J. C. and Rubinstein, M. (2012). Recovering stochastic processes from option prices. In Derivative Securities Pricing and Modelling, vol. 94. Emerald Group Publishing Limited, 123–153.
Johnson, N. (1949). System of frequency curves generated by methods of translation. Biometrika 36 149–176.
Kou, S. G. (2002). A jump-diﬀusion model for option pricing. Management Science 48 1086–1101.
Lin, Y., Lehnert, T. and Wolff, C. (2019). Skewness risk premium: Theory and empirical evidence. International Review of Financial Analysis 63 173–185.
Lo, C., Nguyen, D. and Skindilias, K. (2017). A uniﬁed tree approach for options pricing under stochastic volatility models. Finance Research Letters 20 260–268.
Lu, L., Xu, W. and Qian, Z. (2017a). Eﬃcient convergent lattice method for Asian options pricing with superlinear complexity. Journal of Computational Finance 20 1–38.
Lu, L., Xu, W. and Qian, Z. (2017b). Eﬃcient willow tree method for European-style and American-style moving average barrier options pricing. Quantitative Finance 17 889–906.
Ma, J., Huang, S. and Xu, W. (2020). An eﬃcient convergent willow tree method for american and exotic option pricing under stochastic volatility models. Journal of Derivatives 27 75–98.
Ma, J., Xu, W. and Yao, Y. (2021). Cosine willow tree structure under L´evy processes with application to pricing variance derivatives. Journal of Derivatives 29 30–60.
Merton, R. C. (1976). Option pricing when underlying stock returns are discontinuous. Journal of Financial Economics 3 125–144.
Rubinstein, M. (1994). Implied binomial trees. The Journal of Finance 49 771–818.
Shimko, D. (1993). The bounds of probability. Risk 6 33–37.
Tian, Y. S. (2011). Extracting risk-neutral density and its moments from American option prices. Journal of Derivatives 18 17–34.
Tian, Y. S. (2015). Implied binomial trees with cubic spline smoothing. Journal of Derivatives 22 40–55.
Xu, W., Hong, Z. and Qin, C. (2013). A new sampling strategy willow tree method with application to path-dependent option pricing. Quantitative Finance 13 861–872.
Xu, W. and Yin, Y. (2014). Pricing American options by willow tree method under jump-diﬀusion process. Journal of Derivatives 22 1–9.

## A Proof of Theorem 2.1

**Proof**. Considering the asset price at T , ST , and any twice diﬀerentiable payoﬀ function f (·). From the celebrated Carr-Madan spanning formula (Carr and Madan, 1998, 2001), for all θ ≥ 0, the payoﬀ value f (ST ) can be evaluated as

f (ST ) = f (θ)+f (cid:48)(θ) [(ST − θ)+ − (θ − ST )+]+ (cid:90) θ 0 f (cid:48)(cid:48)(K)(K−ST )+dK+ (cid:90) ∞ θ f (cid:48)(cid:48)(K)(ST −K)+dK. (A.9)

In the absence of arbitrage, a decomposition similar to (A.9) must prevail among the initial values. Let V f 0 and B0(T ) denote the initial values of the derivative with payoﬀ function f (ST ) and the pure discount bond respectively. Then the no arbitrage condition requires that:

V f 0 = B0(T )EQ[f (ST )] = f (θ)B0(T ) + f (cid:48)(θ) [C0(T, θ) − P0(T, θ)] + (cid:82) θ 0 f (cid:48)(cid:48)(K)P0(T, K)dK + (cid:82) ∞ θ f (cid:48)(cid:48)(K)C0(T, K)dK, (A.10)

where C0(T, K) and P0(T, K) are the European call and put option prices at t = 0 with strick price K at maturity T underlying the asset S, respectively. It is obvious that

C0(T, K) − P0(T, K) = B0(T ) {E[(ST − K)+] − E[(K − ST )+]} = B0(T ) ( S0 B0(T ) − K ) , (A.11)

since ST is a martingale under Q measure, i.e., E[ST ] = S0 B0(T ) . Inserting (A.11) into (A.10), we have

EQ[f (ST )] = V f 0 B0(T ) = f (θ)+f (cid:48)(θ) ( S0 B0(T ) − θ ) + (cid:90) θ 0 f (cid:48)(cid:48)(K) ˆP0(T, K)dK+ (cid:90) ∞ θ f (cid:48)(cid:48)(K) ˆC0(T, K)dK, (A.12)

where ˆP0(T, K) and ˆC0(T, K) are the initial futures price of the put and the call, respectively, i.e., ˆP0(T, K) = P0(T, K)/B0(T ) and ˆC0(T, K) = C0(T, K)/B0(T ). Let θ = S0, EQ[f (ST )] in (A.12) can be estimated in a discrete form based on option market prices,

EQ[f (ST )] = f (S0) + f (cid:48)(S0) ( S0 B0(T ) − S0 ) + (cid:88) i ∆Ki B0(T ) f (cid:48)(cid:48)(Ki)Q0(T, Ki), (A.13)

where Q0(T, Ki) is the average price of the bid-ask for each out-of-money call/put option with strike price Ki (Ki > S0 for call; Ki < S0 for put) at maturity T and ∆Ki = (Ki+1 + Ki−1)/2. In fact, (A.13) is also used in the deﬁnition of VIX traded in CBOE. 7 Therefore, the following theorem evaluates the moments of log return process, XT := ln ST S0 based on (A.13).

Inserting f (ST ) = ln ST S0 into (A.12) and (A.13). Let θ = S0, the expectation of the log-return ln ST S0 is

E0 [ln ST S0 ] = ln θ S0 + S0 θB0(T ) − 1 + (cid:82) θ 0 1 K2 ˆP0(T, K)dK + (cid:82) ∞ θ 1 K2 ˆC0(T, K)dK ≈ 1 B0(T ) − 1 + 1 B0(T ) (cid:80) j ∆Kj (Kj )2 Q0(T, Kj). (A.14)

Similarly, when we assume f (ST ) = (ln ST S0 )k, for all k ≥ 2, the k-th order moment of the log-return ln ST S0 is

E0 [(ln ST S0 )k] = (ln θ S0 )k + k (ln θ S0 )k−1 ( S0 B0(T ) − θ ) + (cid:82) θ 0 1 K2 [k(k − 1) (ln K S0 )k−2 − k (ln K S0 )k−1] ˆP0(T, K)dK + (cid:82) ∞ θ 1 K2 [k(k − 1) (ln K S0 )k−2 − k (ln K S0 )k−1] ˆC0(T, K)dK ≈ 1 B0(T ) (cid:80) j ∆Kj (Kj )2 [k(k − 1) (ln Kj S0 )k−2 − k (ln Kj S0 )k−1] Q0(T, Kj). (A.15) (cid:50)

## B Sampling strategy for {(˜zi, pi)}

In order to approximate the standard normal distribution discretely, pairs {(˜zi, pi)} for i = 1, 2, · · · , m, are generated where m is the number of discrete values, ˜zi is the discrete value of the standard normal distribution and pi is the corresponding probability of ˜zi. Xu’s sampling strategy (Xu et al., 2013) ﬁrst generate a sequence of {˜qj} where

˜qj = (j − 0.5)˜γ/m, and ˜qm+1−j = ˜qj, for j = 1, 2, · · · , m/2, 0 ≤ ˜γ ≤ 1,

since the inverse of the standard normal distribution function N −1(˜q) is symmetric around 0.5, we only need to generate half values of probability of ˜qj and mirror the other half. Then, the sequence {pi} is normalized as

pi = ˜qi/ (cid:88)m j=1 ˜qj for i = 1, 2, · · · , m.

After that, the sequence of {˜zi} is the solution of the following nonlinear least squares problem

min ˜zi [ (cid:88)m i=1 pi ˜z4 i − 3 ]2 (B.16)

s.t. (cid:80)m i=1 pi ˜zi = 0, (cid:80)m i=1 pi ˜z2 i = 1, Zi−1 ≤ ˜zi ≤ Zi,

where Zi = N −1((cid:80)i j=1 pj), for i = 1, 2, · · · , m − 1, Z0 = −∞ and Zm = ∞ so that pairs {(˜zi, pi)} satisfy the properties of the standard normal distribution, that is its mean equal to zero, variance equal to one and kurtosis equal to three.

## C Option prices and Greeks

Provided the implied willow tree under the Q measure from the previous subsection, the derivative price and its Greeks (∆, Γ, Θ) can be computed eﬃciently. As known, the value of the derivative at asset price S and time t can be denoted as a function V (S, t). The classic European put option and American put option with strike price, K, and maturity, Tn, can be simply evaluated by the backward induction as

V (S0, 0) = { e−r∆t (cid:80)m i=1 q1 i V 1 i European put max { K − S0, e−r∆t (cid:80)m i=1 q1 i V 1 i } American put,

and

V ni = { e−r∆t (cid:80)m j=1 pnijV n+1 j European put max { K − Sni , e−r∆t (cid:80)m j=1 pnijV n+1 j } American put,

where V N i = (K − SN i )+, for i = 1, 2, · · · .m, n = 1, 2, · · · , N − 1. Other exotic options such as Asian option, moving average barrier option, and variance derivatives can also be priced by the algorithms in Lu et al. (2017a,b) since the implied willow tree has the same structure as the classic model-based ones. We just need to replace the model-based willow tree with the implied willow tree in the algorithms. In addition, the willow tree structure provides an eﬃcient and accurate way to compute Greeks (∆, Γ, Θ) for the derivatives.

The Taylor’s expansion of V (S, t) at current asset price S0 at t = 0 can be written as

V (S, t) = V (S0, 0)+ ∂V (S0, 0) ∂S (S−S0)+ ∂V (S0, 0) ∂t t+ 1 2 ∂2V (S0, 0) ∂S2 (S−S0)2+R(S, t), (C.17)

where R(S, t) is the residual for the Taylor’s expansion, and ∂V (S0,0) ∂S , ∂2V (S0,0) ∂S2 , and ∂V (S0,0) ∂t are the Greeks, ∆, Γ and Θ, respectively. Following the backward induction on the willow tree structure (Lu et al., 2017a,b; Xu et al., 2013), we can obtain not only the value of V (S0, 0), but the values of the derivative on each tree node V (Sni , Tn), as well. Then, applying (C.17) to V (Sni , Tn) on each tree node, we have

V (Sni , Tn) = V (S0, 0)+ ∂V (S0, 0) ∂S (Sni −S0)+ ∂V (S0, 0) ∂t Tn+ 1 2 ∂2V (S0, 0) ∂S2 (Sni −S0)2+R(Sni , Tn),

for i = 1, 2, · · · , m and n = 1, 2, · · · , N . The Greeks, ∆, Γ and Θ, for S0 at t = 0, can be obtained by solving the following linear least squares problem

min ∆, Γ, Θ (cid:88)N n=1 (cid:88)m i=1 [ V (Sni , Tn) − ( V (S0, 0) + ∆(Sni − S0) + Θ · Tn + 1 2 Γ(Sni − S0)2 )]2 . (C.18)

In addition, the above linear least squares problems can also be adopted to compute Greeks for any Sni and Tn on the tree node. In other words, the willow tree structure can provide a dynamic hedging strategy along the evolution of the asset price St.

## D Tables and Figures

**Table 2**: Recovered ﬁrst four moments of log return ln St S0 under Q measure by the implied willow tree method (IWT) with respect to the number of nodes, m, on the synthetic data sets. The benchmark is the exact ﬁrst four moments with a ﬁxed maturity T .

(a) Geometric Brownian motion.
| m | T = 1/4 | T = 2/4 | T = 3/4 | T = 1 |
| :--- | :--- | :--- | :--- | :--- |
| 10 | Mean 0.00149, Var 0.02200, Skew 0.04300, Kurt 2.57187 | Mean 0.00282, Var 0.04434, Skew 0.02352, Kurt 2.64029 | Mean 0.00425, Var 0.06647, Skew 0.02914, Kurt 2.63604 | Mean 0.00560, Var 0.08884, Skew 0.02164, Kurt 2.65279 |
| 20 | Mean 0.00130, Var 0.02238, Skew 0.01622, Kurt 2.84599 | Mean 0.00262, Var 0.04471, Skew 0.02249, Kurt 2.83377 | Mean 0.00388, Var 0.06725, Skew 0.00802, Kurt 2.85813 | Mean 0.00515, Var 0.08972, Skew 0.00762, Kurt 2.86874 |
| 30 | Mean 0.00125, Var 0.02250, Skew -0.00307, Kurt 2.94654 | Mean 0.00254, Var 0.04492, Skew 0.00405, Kurt 2.92367 | Mean 0.00382, Var 0.06734, Skew 0.00790, Kurt 2.91423 | Mean 0.00512, Var 0.08972, Skew 0.01182, Kurt 2.90577 |
| 50 | Mean 0.00127, Var 0.02247, Skew 0.00314, Kurt 2.92764 | Mean 0.00253, Var 0.04492, Skew 0.00628, Kurt 2.93352 | Mean 0.00377, Var 0.06747, Skew 0.00192, Kurt 2.96356 | Mean 0.00502, Var 0.08994, Skew 0.00346, Kurt 2.96160 |
| BENCHMARK | Mean 0.00125, Var 0.0225, Skew 0, Kurt 3 | Mean 0.0025, Var 0.045, Skew 0, Kurt 3 | Mean 0.00375, Var 0.0675, Skew 0, Kurt 3 | Mean 0.005, Var 0.09, Skew 0, Kurt 3 |

(b) Merton’s jump-diﬀusion model.
| m | T = 1/4 | T = 2/4 | T = 3/4 | T = 1 |
| :--- | :--- | :--- | :--- | :--- |
| 10 | Mean 0.00856, Var 0.00825, Skew -1.55950, Kurt 5.46001 | Mean 0.01674, Var 0.01736, Skew -1.14943, Kurt 4.25775 | Mean 0.02498, Var 0.02633, Skew -0.94295, Kurt 3.73312 | Mean 0.03325, Var 0.03523, Skew -0.81288, Kurt 3.44620 |
| 20 | Mean 0.00831, Var 0.00888, Skew -1.94738, Kurt 8.15846 | Mean 0.01648, Var 0.01806, Skew -1.33173, Kurt 5.44838 | Mean 0.02472, Var 0.02708, Skew -1.08895, Kurt 4.56754 | Mean 0.03294, Var 0.03612, Skew -0.93022, Kurt 4.10859 |
| 30 | Mean 0.00830, Var 0.00890, Skew -1.94820, Kurt 8.51761 | Mean 0.01655, Var 0.01784, Skew -1.25256, Kurt 4.98936 | Mean 0.02479, Var 0.02689, Skew -1.05349, Kurt 4.47251 | Mean 0.03295, Var 0.03610, Skew -0.92610, Kurt 4.20167 |
| 50 | Mean 0.00832, Var 0.00886, Skew -1.93425, Kurt 8.30237 | Mean 0.01650, Var 0.01797, Skew -1.28561, Kurt 5.30999 | Mean 0.02466, Var 0.02719, Skew -1.08492, Kurt 4.64948 | Mean 0.03292, Var 0.03614, Skew -0.91950, Kurt 4.12338 |
| BENCHMARK | Mean 0.00822, Var 0.00908, Skew -1.92743, Kurt 9.29430 | Mean 0.01643, Var 0.01816, Skew -1.36290, Kurt 6.14715 | Mean 0.02465, Var 0.02724, Skew -1.11280, Kurt 5.09810 | Mean 0.03287, Var 0.03632, Skew -0.96371, Kurt 4.57357 |

(c) Heston’s stochastic volatility model.
| m | T = 1/4 | T = 2/4 | T = 3/4 | T = 1 |
| :--- | :--- | :--- | :--- | :--- |
| 10 | Mean 0.00637, Var 0.00734, Skew -0.44277, Kurt 2.80119 | Mean 0.01333, Var 0.01362, Skew -0.52912, Kurt 2.92456 | Mean 0.02025, Var 0.01995, Skew -0.47680, Kurt 2.95815 | Mean 0.02720, Var 0.02620, Skew -0.42393, Kurt 2.92283 |
| 20 | Mean 0.00629, Var 0.00755, Skew -0.60611, Kurt 3.31272 | Mean 0.01318, Var 0.01398, Skew -0.61183, Kurt 3.50016 | Mean 0.02009, Var 0.02034, Skew -0.54539, Kurt 3.42487 | Mean 0.02704, Var 0.02658, Skew -0.47667, Kurt 3.28920 |
| 30 | Mean 0.00621, Var 0.00775, Skew -0.78214, Kurt 4.10197 | Mean 0.01309, Var 0.01419, Skew -0.69235, Kurt 4.02437 | Mean 0.02005, Var 0.02044, Skew -0.57271, Kurt 3.60080 | Mean 0.02701, Var 0.02667, Skew -0.49391, Kurt 3.42100 |
| 50 | Mean 0.00619, Var 0.00780, Skew -0.81466, Kurt 4.27095 | Mean 0.01307, Var 0.01424, Skew -0.70840, Kurt 4.23717 | Mean 0.01995, Var 0.02070, Skew -0.65683, Kurt 4.18943 | Mean 0.02687, Var 0.02708, Skew -0.58785, Kurt 3.92897 |
| BENCHMARK | Mean 0.00626, Var 0.00778, Skew -0.72582, Kurt 4.27387 | Mean 0.01316, Var 0.01415, Skew -0.67096, Kurt 4.10113 | Mean 0.02005, Var 0.02051, Skew -0.59803, Kurt 3.86459 | Mean 0.02698, Var 0.02684, Skew -0.53439, Kurt 3.67530 |

**Table 3**: Recovered ﬁrst four moments of log return ln St S0 under Q measure by the implied willow tree method (IWT, m = 30) with respect to the number of strike prices, NK, on the synthetic data sets. The benchmark is the exact ﬁrst four moments with a ﬁxed maturity T .

(a) Geometric Brownian motion.
| NK | T = 1/4 | T = 2/4 | T = 3/4 | T = 1 |
| :--- | :--- | :--- | :--- | :--- |
| 7 | Mean 0.00134, Var 0.02233, Skew -0.00485, Kurt 2.79226 | Mean 0.00286, Var 0.04443, Skew -0.03036, Kurt 2.65666 | Mean 0.00490, Var 0.06571, Skew -0.05382, Kurt 2.42691 | Mean 0.00704, Var 0.08606, Skew -0.07793, Kurt 2.18016 |
| 11 | Mean 0.00133, Var 0.02234, Skew 0.00861, Kurt 2.82622 | Mean 0.00258, Var 0.04485, Skew 0.00476, Kurt 2.86027 | Mean 0.00391, Var 0.06727, Skew -0.00405, Kurt 2.82158 | Mean 0.00557, Var 0.08914, Skew -0.01119, Kurt 2.73163 |
| 21 | Mean 0.00131, Var 0.02238, Skew 0.00867, Kurt 2.84942 | Mean 0.00260, Var 0.04481, Skew 0.00324, Kurt 2.85006 | Mean 0.00396, Var 0.06719, Skew -0.00690, Kurt 2.79552 | Mean 0.00564, Var 0.08908, Skew -0.01672, Kurt 2.68667 |
| 40 | Mean 0.00133, Var 0.02233, Skew 0.01240, Kurt 2.82496 | Mean 0.00257, Var 0.04483, Skew 0.01648, Kurt 2.86321 | Mean 0.00387, Var 0.06727, Skew 0.00718, Kurt 2.87834 | Mean 0.00516, Var 0.08969, Skew 0.00861, Kurt 2.88388 |
| 50 | Mean 0.00125, Var 0.02251, Skew 0.00192, Kurt 2.91620 | Mean 0.00262, Var 0.04474, Skew 0.01120, Kurt 2.87842 | Mean 0.00396, Var 0.06705, Skew 0.01351, Kurt 2.87105 | Mean 0.00518, Var 0.08969, Skew 0.00444, Kurt 2.87078 |
| 100 | Mean 0.00131, Var 0.02237, Skew 0.01262, Kurt 2.84483 | Mean 0.00259, Var 0.04480, Skew 0.01460, Kurt 2.86170 | Mean 0.00385, Var 0.06731, Skew 0.00632, Kurt 2.86704 | Mean 0.00514, Var 0.08975, Skew 0.00650, Kurt 2.87166 |
| 200 | Mean 0.00130, Var 0.02239, Skew 0.01572, Kurt 2.85082 | Mean 0.00262, Var 0.04472, Skew 0.02216, Kurt 2.83405 | Mean 0.00389, Var 0.06721, Skew 0.01179, Kurt 2.85348 | Mean 0.00514, Var 0.08974, Skew 0.00713, Kurt 2.87006 |
| 400 | Mean 0.00130, Var 0.02238, Skew 0.01622, Kurt 2.84599 | Mean 0.00262, Var 0.04471, Skew 0.02249, Kurt 2.83377 | Mean 0.00389, Var 0.06720, Skew 0.01259, Kurt 2.85103 | Mean 0.00515, Var 0.08972, Skew 0.00763, Kurt 2.86875 |
| BENCHMARK | Mean 0.00125, Var 0.0225, Skew 0, Kurt 3 | Mean 0.0025, Var 0.045, Skew 0, Kurt 3 | Mean 0.00375, Var 0.0675, Skew 0, Kurt 3 | Mean 0.005, Var 0.09, Skew 0, Kurt 3 |

(b) Merton’s jump-diﬀusion model.
| NK | T = 1/4 | T = 2/4 | T = 3/4 | T = 1 |
| :--- | :--- | :--- | :--- | :--- |
| 7 | Mean 0.00827, Var 0.00894, Skew -1.82580, Kurt 7.98947 | Mean 0.01650, Var 0.01798, Skew -1.29028, Kurt 5.37759 | Mean 0.02475, Var 0.02696, Skew -1.05395, Kurt 4.47366 | Mean 0.03308, Var 0.03576, Skew -0.89258, Kurt 3.91421 |
| 11 | Mean 0.00826, Var 0.00898, Skew -1.84036, Kurt 8.05442 | Mean 0.01650, Var 0.01800, Skew -1.31670, Kurt 5.53637 | Mean 0.02472, Var 0.02706, Skew -1.07779, Kurt 4.62999 | Mean 0.03297, Var 0.03603, Skew -0.91772, Kurt 4.14687 |
| 21 | Mean 0.00828, Var 0.00894, Skew -1.90022, Kurt 8.29278 | Mean 0.01654, Var 0.01791, Skew -1.30389, Kurt 5.34290 | Mean 0.02475, Var 0.02697, Skew -1.06186, Kurt 4.53515 | Mean 0.03296, Var 0.03605, Skew -0.91422, Kurt 4.12216 |
| 40 | Mean 0.00826, Var 0.00897, Skew -1.83889, Kurt 8.04939 | Mean 0.01656, Var 0.01786, Skew -1.29623, Kurt 5.27808 | Mean 0.02476, Var 0.02695, Skew -1.05856, Kurt 4.49269 | Mean 0.03288, Var 0.03632, Skew -0.96231, Kurt 4.36414 |
| 50 | Mean 0.00841, Var 0.00873, Skew -2.19832, Kurt 9.22765 | Mean 0.01646, Var 0.01811, Skew -1.36976, Kurt 5.76933 | Mean 0.02466, Var 0.02723, Skew -1.11002, Kurt 4.84764 | Mean 0.03295, Var 0.03610, Skew -0.92860, Kurt 4.21151 |
| 100 | Mean 0.00836, Var 0.00877, Skew -1.89997, Kurt 7.80398 | Mean 0.01653, Var 0.01791, Skew -1.30328, Kurt 5.36598 | Mean 0.02468, Var 0.02716, Skew -1.08865, Kurt 4.72053 | Mean 0.03294, Var 0.03612, Skew -0.92899, Kurt 4.20883 |
| 200 | Mean 0.00835, Var 0.00879, Skew -1.90861, Kurt 7.90334 | Mean 0.01655, Var 0.01785, Skew -1.25488, Kurt 5.00257 | Mean 0.02478, Var 0.02689, Skew -1.05261, Kurt 4.47049 | Mean 0.03295, Var 0.03609, Skew -0.92586, Kurt 4.19809 |
| 400 | Mean 0.00830, Var 0.00890, Skew -1.94979, Kurt 8.52617 | Mean 0.01655, Var 0.01784, Skew -1.25253, Kurt 4.98929 | Mean 0.02479, Var 0.02688, Skew -1.05338, Kurt 4.47139 | Mean 0.03295, Var 0.03610, Skew -0.92615, Kurt 4.20192 |
| BENCHMARK | Mean 0.00822, Var 0.00908, Skew -1.92743, Kurt 9.29430 | Mean 0.01643, Var 0.01816, Skew -1.36290, Kurt 6.14715 | Mean 0.02465, Var 0.02724, Skew -1.11280, Kurt 5.09810 | Mean 0.03287, Var 0.03632, Skew -0.96371, Kurt 4.57357 |

(c) Heston’s stochastic volatility model.
| NK | T = 1/4 | T = 2/4 | T = 3/4 | T = 1 |
| :--- | :--- | :--- | :--- | :--- |
| 7 | Mean 0.00620, Var 0.00777, Skew -0.72102, Kurt 3.86185 | Mean 0.01313, Var 0.01408, Skew -0.64163, Kurt 3.75766 | Mean 0.02005, Var 0.02045, Skew -0.57595, Kurt 3.58374 | Mean 0.02702, Var 0.02668, Skew -0.50555, Kurt 3.40379 |
| 11 | Mean 0.00620, Var 0.00777, Skew -0.72081, Kurt 3.86665 | Mean 0.01312, Var 0.01410, Skew -0.64318, Kurt 3.79675 | Mean 0.02003, Var 0.02049, Skew -0.58163, Kurt 3.64182 | Mean 0.02700, Var 0.02671, Skew -0.50406, Kurt 3.45875 |
| 21 | Mean 0.00618, Var 0.00782, Skew -0.75945, Kurt 4.25374 | Mean 0.01312, Var 0.01410, Skew -0.64868, Kurt 3.78966 | Mean 0.02004, Var 0.02046, Skew -0.57511, Kurt 3.62656 | Mean 0.02699, Var 0.02672, Skew -0.50267, Kurt 3.44879 |
| 40 | Mean 0.00622, Var 0.00774, Skew -0.78461, Kurt 4.02789 | Mean 0.01310, Var 0.01418, Skew -0.68871, Kurt 4.00589 | Mean 0.02004, Var 0.02048, Skew -0.58559, Kurt 3.60663 | Mean 0.02699, Var 0.02672, Skew -0.50471, Kurt 3.46436 |
| 50 | Mean 0.00617, Var 0.00782, Skew -0.72107, Kurt 4.01381 | Mean 0.01314, Var 0.01405, Skew -0.61702, Kurt 3.67988 | Mean 0.02005, Var 0.02043, Skew -0.55907, Kurt 3.56982 | Mean 0.02699, Var 0.02674, Skew -0.50428, Kurt 3.42683 |
| 100 | Mean 0.00621, Var 0.00774, Skew -0.77201, Kurt 4.06627 | Mean 0.01312, Var 0.01410, Skew -0.64604, Kurt 3.80425 | Mean 0.02006, Var 0.02043, Skew -0.57561, Kurt 3.56725 | Mean 0.02700, Var 0.02669, Skew -0.49526, Kurt 3.43163 |
| 200 | Mean 0.00621, Var 0.00775, Skew -0.77138, Kurt 4.06028 | Mean 0.01309, Var 0.01419, Skew -0.69365, Kurt 4.02918 | Mean 0.02005, Var 0.02043, Skew -0.57152, Kurt 3.59883 | Mean 0.02701, Var 0.02667, Skew -0.49400, Kurt 3.42389 |
| 400 | Mean 0.00621, Var 0.00775, Skew -0.78247, Kurt 4.10313 | Mean 0.01309, Var 0.01419, Skew -0.69234, Kurt 4.02431 | Mean 0.02005, Var 0.02044, Skew -0.57271, Kurt 3.60079 | Mean 0.02701, Var 0.02667, Skew -0.49369, Kurt 3.42062 |
| BENCHMARK | Mean 0.00626, Var 0.00778, Skew -0.72582, Kurt 4.27387 | Mean 0.01316, Var 0.01415, Skew -0.67096, Kurt 4.10113 | Mean 0.02005, Var 0.02051, Skew -0.59803, Kurt 3.86459 | Mean 0.02698, Var 0.02684, Skew -0.53439, Kurt 3.67530 |

**Table 4**: Recovered moments of log return under the Q measure via the implied willow tree method (IWT, m = 30) with various parameters. The benchmark is the exact ﬁrst four moments for each maturity T .

(a) Geometric Brownian motion.
| | σ = 0.4 | σ = 0.6 | σ = 0.8 |
| :--- | :--- | :--- | :--- |
| T = 1/4 | BENCHMARK: Mean -0.0075, Var 0.04, Skew 0, Kurt 3 | BENCHMARK: Mean -0.0325, Var 0.09, Skew 0, Kurt 3 | BENCHMARK: Mean -0.0675, Var 0.16, Skew 0, Kurt 3 |
| | IWT: Mean -0.00747, Var 0.03994, Skew 0.00319, Kurt 2.92602 | IWT: Mean -0.03238, Var 0.08972, Skew 0.01194, Kurt 2.90600 | IWT: Mean -0.06734, Var 0.15966, Skew 0.00925, Kurt 2.91916 |
| T = 2/4 | BENCHMARK: Mean -0.015, Var 0.08, Skew 0, Kurt 3 | BENCHMARK: Mean -0.065, Var 0.18, Skew 0, Kurt 3 | BENCHMARK: Mean -0.135, Var 0.32, Skew 0, Kurt 3 |
| | IWT: Mean -0.01490, Var 0.07977, Skew 0.01031, Kurt 2.90918 | IWT: Mean -0.06483, Var 0.17966, Skew 0.00887, Kurt 2.91980 | IWT: Mean -0.13481, Var 0.31986, Skew 0.00627, Kurt 2.92983 |
| T = 3/4 | BENCHMARK: Mean -0.0225, Var 0.12, Skew 0, Kurt 3 | BENCHMARK: Mean -0.0975, Var 0.27, Skew 0, Kurt 3 | BENCHMARK: Mean -0.2025, Var 0.48, Skew 0, Kurt 3 |
| | IWT: Mean -0.02240, Var 0.11979, Skew 0.00670, Kurt 2.92270 | IWT: Mean -0.09732, Var 0.26979, Skew 0.00641, Kurt 2.92838 | IWT: Mean -0.20211, Var 0.47986, Skew 0.00892, Kurt 2.92210 |
| T = 1 | BENCHMARK: Mean -0.03, Var 0.16, Skew 0, Kurt 3 | BENCHMARK: Mean -0.13, Var 0.36, Skew 0, Kurt 3 | BENCHMARK: Mean -0.27, Var 0.64, Skew 0, Kurt 3 |
| | IWT: Mean -0.02984, Var 0.15965, Skew 0.00933, Kurt 2.91903 | IWT: Mean -0.12978, Var 0.35995, Skew 0.00614, Kurt 2.92785 | IWT: Mean -0.26924, Var 0.63999, Skew 0.01296, Kurt 2.90325 |

(b) Merton’s jump-diﬀusion model.
| | λ = 1 | λ = 2 | λ = 5 |
| :--- | :--- | :--- | :--- |
| T = 1/4 | BENCHMARK: Mean 0.00578, Var 0.01442, Skew -1.82339, Kurt 7.72517 | BENCHMARK: Mean 0.00061, Var 0.02574, Skew -1.52933, Kurt 5.96644 | BENCHMARK: Mean -0.01491, Var 0.05970, Skew -1.08252, Kurt 4.37879 |
| | IWT: Mean 0.00591, Var 0.01409, Skew -1.71658, Kurt 6.12862 | IWT: Mean 0.00080, Var 0.02518, Skew -1.40439, Kurt 4.77670 | IWT: Mean -0.01484, Var 0.05940, Skew -1.04373, Kurt 4.05080 |
| T = 2/4 | BENCHMARK: Mean 0.01155, Var 0.02884, Skew -1.28933, Kurt 5.36258 | BENCHMARK: Mean 0.00121, Var 0.05148, Skew -1.08140, Kurt 4.48322 | BENCHMARK: Mean -0.02982, Var 0.11939, Skew -0.76546, Kurt 3.68940 |
| | IWT: Mean 0.01167, Var 0.02851, Skew -1.22499, Kurt 4.71898 | IWT: Mean 0.00131, Var 0.05120, Skew -1.05104, Kurt 4.16095 | IWT: Mean -0.02968, Var 0.11884, Skew -0.73465, Kurt 3.45597 |
| T = 3/4 | BENCHMARK: Mean 0.01733, Var 0.04326, Skew -1.05274, Kurt 4.57506 | BENCHMARK: Mean 0.00182, Var 0.07722, Skew -0.88296, Kurt 3.98881 | BENCHMARK: Mean -0.04472, Var 0.17909, Skew -0.62499, Kurt 3.45960 |
| | IWT: Mean 0.01743, Var 0.04296, Skew -1.01015, Kurt 4.18200 | IWT: Mean 0.00194, Var 0.07676, Skew -0.84424, Kurt 3.68460 | IWT: Mean -0.04331, Var 0.17535, Skew -0.58161, Kurt 3.14664 |
| T = 1 | BENCHMARK: Mean 0.02311, Var 0.05768, Skew -0.91170, Kurt 4.18129 | BENCHMARK: Mean 0.00242, Var 0.10296, Skew -0.76467, Kurt 3.74161 | BENCHMARK: Mean -0.05963, Var 0.23878, Skew -0.54126, Kurt 3.34470 |
| | IWT: Mean 0.02320, Var 0.05739, Skew -0.87831, Kurt 3.86503 | IWT: Mean 0.00260, Var 0.10226, Skew -0.72297, Kurt 3.46349 | IWT: Mean -0.05069, Var 0.21886, Skew -0.50022, Kurt 2.79781 |

(c) Heston’s stochastic volatility model.
| | Set 1 | Set 2 | Set 3 |
| :--- | :--- | :--- | :--- |
| T = 1/4 | BENCHMARK: Mean 0.00547, Var 0.00902, Skew -0.71718, Kurt 3.97129 | BENCHMARK: Mean 0.00626, Var 0.00778, Skew -0.72582, Kurt 4.27387 | BENCHMARK: Mean -0.00287, Var 0.02490, Skew 0.05766, Kurt 3.62983 |
| | IWT: Mean 0.00563, Var 0.00896, Skew -0.77279, Kurt 3.97006 | IWT: Mean 0.00621, Var 0.00775, Skew -0.78214, Kurt 4.10197 | IWT: Mean -0.00257, Var 0.02505, Skew 0.04783, Kurt 3.40920 |
| T = 2/4 | BENCHMARK: Mean 0.01121, Var 0.01872, Skew -0.88605, Kurt 4.53740 | BENCHMARK: Mean 0.01316, Var 0.01415, Skew -0.67096, Kurt 4.10113 | BENCHMARK: Mean -0.01135, Var 0.06190, Skew 0.03527, Kurt 3.54926 |
| | IWT: Mean 0.01093, Var 0.01889, Skew -0.92734, Kurt 4.56692 | IWT: Mean 0.01309, Var 0.01419, Skew -0.69235, Kurt 4.02437 | IWT: Mean -0.01125, Var 0.06218, Skew 0.03594, Kurt 3.38421 |
| T = 3/4 | BENCHMARK: Mean 0.01586, Var 0.02952, Skew -0.99836, Kurt 4.98970 | BENCHMARK: Mean 0.02005, Var 0.02051, Skew -0.59803, Kurt 3.86459 | BENCHMARK: Mean -0.02280, Var 0.10554, Skew 0.02516, Kurt 3.48917 |
| | IWT: Mean 0.01612, Var 0.02925, Skew -0.96304, Kurt 4.55083 | IWT: Mean 0.02005, Var 0.02044, Skew -0.57271, Kurt 3.60080 | IWT: Mean -0.02283, Var 0.10502, Skew 0.03285, Kurt 3.29964 |
| T = 1 | BENCHMARK: Mean 0.02105, Var 0.04126, Skew -1.07062, Kurt 5.30003 | BENCHMARK: Mean 0.02698, Var 0.02684, Skew -0.53439, Kurt 3.67530 | BENCHMARK: Mean -0.03713, Var 0.14984, Skew -0.0223, Kurt 3.43444 |
| | IWT: Mean 0.02102, Var 0.04044, Skew -0.99681, Kurt 4.69142 | IWT: Mean 0.02701, Var 0.02667, Skew -0.49391, Kurt 3.42100 | IWT: Mean -0.03591, Var 0.15067, Skew -0.03885, Kurt 3.20946 |

**Figure 2**: Computed European, American, Asian option prices and Greeks, ∆ and Γ of European put options via the implied willow tree method (IWT, m = 30) with respect to strikes using the synthetic data generated by geometric Brownian motion. The benchmark is the exact values of options or Greeks for each maturity T .

**Figure 3**: Computed European, American, Asian option prices and Greeks, ∆ and Γ of European put options via the implied willow tree method (IWT, m = 30) with respect to strikes using the synthetic data generated by Merton’s jump-diﬀusion model. The benchmark is the exact values of options or Greeks for each maturity T .

**Figure 4**: Computed European, American, Asian option prices and Greeks, ∆ and Γ of European put options via the implied willow tree method (IWT, m = 30) with respect to strikes using the synthetic data generated by Heston’s stochastic volatility model. The benchmark is the exact values of options or Greeks for each maturity T .

**Table 5**: Recovered moments of log return under the Q measure via the implied willow tree method (IWT, m = 30) on three synthetic data sets with perturbed noise. The benchmark is the exact ﬁrst four moments for each maturity T .

(a) Geometric Brownian motion.
| | IWT without noise | IWT with noise | BENCHMARK |
| :--- | :--- | :--- | :--- |
| T = 1/4 | Mean 0.00125, Var 0.02250, Skew -0.00307, Kurt 2.94654 | Mean 0.00099, Var 0.02339, Skew 0.00122, Kurt 3.09419 | Mean 0.00125, Var 0.0225, Skew 0, Kurt 3 |
| T = 2/4 | Mean 0.00254, Var 0.04492, Skew 0.00411, Kurt 2.92366 | Mean 0.00356, Var 0.04294, Skew 0.00504, Kurt 2.83054 | Mean 0.0025, Var 0.045, Skew 0, Kurt 3 |
| T = 1 | Mean 0.00512, Var 0.08972, Skew 0.01185, Kurt 2.90581 | Mean 0.00662, Var 0.08652, Skew 0.06597, Kurt 2.83390 | Mean 0.005, Var 0.09, Skew 0, Kurt 3 |

(b) Merton’s jump-diﬀusion model.
| | IWT without noise | IWT with noise | BENCHMARK |
| :--- | :--- | :--- | :--- |
| T = 1/4 | Mean 0.00830, Var 0.00890, Skew -1.94820, Kurt 8.51761 | Mean 0.00806, Var 0.00951, Skew -2.20309, Kurt 9.70242 | Mean 0.00822, Var 0.00908, Skew -1.92743, Kurt 9.29430 |
| T = 2/4 | Mean 0.01655, Var 0.01784, Skew -1.25256, Kurt 4.98936 | Mean 0.01658, Var 0.01769, Skew -1.14487, Kurt 4.40095 | Mean 0.01643, Var 0.01816, Skew -1.36290, Kurt 6.14715 |
| T = 1 | Mean 0.03295, Var 0.03610, Skew -0.92610, Kurt 4.20167 | Mean 0.03220, Var 0.03804, Skew -1.05205, Kurt 4.31812 | Mean 0.03287, Var 0.03632, Skew -0.96371, Kurt 4.57357 |

(c) Heston’s stochastic volatility model.
| | IWT without noise | IWT with noise | BENCHMARK |
| :--- | :--- | :--- | :--- |
| T = 1/4 | Mean 0.00621, Var 0.00775, Skew -0.78214, Kurt 4.10197 | Mean 0.00625, Var 0.00767, Skew -0.81119, Kurt 4.22278 | Mean 0.00626, Var 0.00778, Skew -0.72582, Kurt 4.27387 |
| T = 2/4 | Mean 0.01309, Var 0.01419, Skew -0.69235, Kurt 4.02437 | Mean 0.01344, Var 0.01335, Skew -0.45650, Kurt 3.06242 | Mean 0.01316, Var 0.01415, Skew -0.67096, Kurt 4.10113 |
| T = 1 | Mean 0.02701, Var 0.02667, Skew -0.49391, Kurt 3.42100 | Mean 0.02725, Var 0.02618, Skew -0.48826, Kurt 3.04610 | Mean 0.02698, Var 0.02684, Skew -0.53439, Kurt 3.67530 |

**Figure 5**: Computed European, American and Asian option prices via the implied willow tree method (IWT, m = 30) with respect to strikes based on the perturbed synthetic data generated by geometric Brownian motion. The benchmark is the exact values of options computed analytically or by Monte Carlo simulations with 100,000 paths.

**Figure 6**: Computed European, American and Asian option prices via the implied willow tree method (IWT, m = 30) with respect to strikes based on the perturbed synthetic data generated by Merton’s jump diﬀusion model. The benchmark is the exact values of options computed analytically or by Monte Carlo simulations with 100,000 paths.

**Figure 7**: Computed European, American and Asian option prices via the implied willow tree method (IWT, m = 30) with respect to strikes based on the perturbed synthetic data generated by Heston’s stochastic volatility model. The benchmark is the exact values of options computed analytically or by Monte Carlo simulations with 100,000 paths.

**Table 6**: Statistics of S&P 500 index option prices from January 1, 2006 to December 31, 2019 (Data source: CBOE Data Shop (https://datashop.cboe.com)).

| | Daily | Weekly | Monthly |
| :--- | :--- | :--- | :--- |
| Call | # of prices 1,743,430, Mean 57.7587, Std. Dev 392.9030, Skew 7.7447, Kurt 89.4872 | # of prices 328,239, Mean 57.9605, Std. Dev 392.5803, Skew 7.5629, Kurt 85.0386 | # of prices 83,361, Mean 58.3779, Std. Dev 391.5588, Skew 7.5009, Kurt 83.7037 |
| Put | # of prices 2,860,499, Mean 26.2860, Std. Dev 153.3457, Skew 10.8493, Kurt 198.7755 | # of prices 539,325, Mean 25.7747, Std. Dev 153.7834, Skew 11.0561, Kurt 205.2392 | # of prices 135,566, Mean 26.5822, Std. Dev 153.5725, Skew 11.3082, Kurt 213.5633 |

**Figure 8**: Discrete probabilities between 5th and 95th percentile of returns (ST /S0) under the risk-neutral measure with α = 100S0 and S0 = 2854.53 in one week, two weeks and three weeks on S&P 500 index at May 15, 2019.

**Table 7**: Mean relative error (MRE) and cent error of computed S&P 500 index call/put option prices compared with the weekly market prices in the entire period (2006-2019) and three sub-periods (2006-2010, 2011-2014, 2015-2019). The average call and put market prices are 57.96 and 25.77 dollars between 2006 and 2019, respectively.

(a) Call Option
| | 2006 - 2019 | 2006 - 2010 | 2011 - 2014 | 2015 - 2019 |
| :--- | :--- | :--- | :--- | :--- |
| Overall | MRE 0.0510, Cent Error 176.10 | MRE 0.0381, Cent Error 159.51 | MRE 0.0526, Cent Error 194.14 | MRE 0.0531, Cent Error 174.75 |
| Moneyness (K/S0) | | | | |
| ≤ 0.8 | MRE 0.0049, Cent Error 291.97 | MRE 0.0068, Cent Error 270.48 | MRE 0.0053, Cent Error 284.99 | MRE 0.0030, Cent Error 312.93 |
| (0.8, 0.95] | MRE 0.0162, Cent Error 259.72 | MRE 0.0194, Cent Error 247.83 | MRE 0.0174, Cent Error 250.54 | MRE 0.0133, Cent Error 272.94 |
| (0.95, 1.05] | MRE 0.0508, Cent Error 179.14 | MRE 0.0412, Cent Error 172.12 | MRE 0.0546, Cent Error 198.28 | MRE 0.0512, Cent Error 175.74 |
| (1.05, 1.2] | MRE 0.0626, Cent Error 111.10 | MRE 0.0346, Cent Error 81.18 | MRE 0.0598, Cent Error 121.69 | MRE 0.0788, Cent Error 121.36 |
| > 1.2 | MRE 0.0997, Cent Error 85.66 | MRE 0.0847, Cent Error 77.17 | MRE 0.1231, Cent Error 71.97 | MRE 0.2040, Cent Error 143.15 |
| Maturity (days) | | | | |
| ≤ 30 | MRE 0.0427, Cent Error 144.25 | MRE 0.0375, Cent Error 105.68 | MRE 0.0401, Cent Error 128.52 | MRE 0.0435, Cent Error 149.09 |
| (30, 90] | MRE 0.0576, Cent Error 199.65 | MRE 0.0354, Cent Error 161.91 | MRE 0.0609, Cent Error 229.28 | MRE 0.0639, Cent Error 201.56 |
| (90, 180] | MRE 0.0657, Cent Error 210.65 | MRE 0.0430, Cent Error 185.20 | MRE 0.0670, Cent Error 234.49 | MRE 0.0747, Cent Error 210.62 |
| > 180 | MRE 0.0697, Cent Error 211.41 | MRE 0.0482, Cent Error 203.38 | MRE 0.0731, Cent Error 213.17 | MRE 0.0790, Cent Error 214.56 |

(b) Put Option
| | 2006 - 2019 | 2006 - 2010 | 2011 - 2014 | 2015 - 2019 |
| :--- | :--- | :--- | :--- | :--- |
| Overall | MRE 0.0242, Cent Error 115.80 | MRE 0.0276, Cent Error 133.81 | MRE 0.0253, Cent Error 118.30 | MRE 0.0235, Cent Error 112.60 |
| Moneyness (K/S0) | | | | |
| ≤ 0.8 | MRE 0.0381, Cent Error 28.46 | MRE 0.0409, Cent Error 51.80 | MRE 0.0265, Cent Error 23.71 | MRE 0.0402, Cent Error 22.17 |
| (0.8, 0.95] | MRE 0.0189, Cent Error 49.18 | MRE 0.0191, Cent Error 63.04 | MRE 0.0185, Cent Error 61.19 | MRE 0.0190, Cent Error 43.72 |
| (0.95, 1.05] | MRE 0.0265, Cent Error 149.90 | MRE 0.0345, Cent Error 167.64 | MRE 0.0316, Cent Error 154.80 | MRE 0.0247, Cent Error 146.91 |
| (1.05, 1.2] | MRE 0.0166, Cent Error 253.17 | MRE 0.0181, Cent Error 232.06 | MRE 0.0210, Cent Error 272.37 | MRE 0.0136, Cent Error 266.70 |
| > 1.2 | MRE 0.0065, Cent Error 255.51 | MRE 0.0068, Cent Error 253.46 | MRE 0.0044, Cent Error 237.39 | MRE 0.0045, Cent Error 314.90 |
| Maturity (days) | | | | |
| ≤ 30 | MRE 0.0230, Cent Error 97.18 | MRE 0.0271, Cent Error 97.50 | MRE 0.0249, Cent Error 93.54 | MRE 0.0224, Cent Error 97.67 |
| (30, 90] | MRE 0.0248, Cent Error 122.41 | MRE 0.0249, Cent Error 124.35 | MRE 0.0255, Cent Error 115.61 | MRE 0.0245, Cent Error 123.60 |
| (90, 180] | MRE 0.0266, Cent Error 134.21 | MRE 0.0348, Cent Error 165.92 | MRE 0.0266, Cent Error 153.10 | MRE 0.0249, Cent Error 121.64 |
| > 180 | MRE 0.0340, Cent Error 146.26 | MRE 0.0423, Cent Error 181.68 | MRE 0.0338, Cent Error 149.62 | MRE 0.0318, Cent Error 133.48 |

**Table 8**: Values of the density smooth term (Smooth) and MSE term (MSE) of the objective function in (4.8) with various α’s on S&P 500 index option prices at May 15, 2019 with S0 = 2854.53.

| α | T = 1/52 | T = 2/52 | T = 3/52 |
| :--- | :--- | :--- | :--- |
| S0*0.1 | MSE 0.3222, Smooth 0.0294 | MSE 0.6658, Smooth 0.0301 | MSE 2.0498, Smooth 0.0477 |
| S0 | MSE 0.3894, Smooth 0.0891 | MSE 0.7430, Smooth 0.0483 | MSE 2.1594, Smooth 0.0692 |
| S0*10 | MSE 0.6134, Smooth 0.1248 | MSE 0.8368, Smooth 0.0867 | MSE 2.3065, Smooth 0.0956 |
| S0*100 | MSE 0.7722, Smooth 0.5227 | MSE 0.8988, Smooth 0.6272 | MSE 2.3889, Smooth 0.6139 |
| S0*1000 | MSE 1.8196, Smooth 3.0151 | MSE 1.8599, Smooth 4.3261 | MSE 3.1808, Smooth 4.4971 |

**Figure 9**: Implied willow tree (IWT) with 30 nodes for discrete maturities of the S&P 500 index based on option data from May 15, 2019, extending up to two weeks. S0 = 2854. [S1 i ] and [S2 j ] are 30 possible index values for one week (t1) and two weeks (t2), respectively. The vector [q1 i ] corresponds to the associated probabilities for each value in [S1 i ] while the matrix [p1 ij] characterizes the transition probabilities from [S1 i ] to [S2 j ]. The row enclosed within the red box represents the transition probabilities associated with 30 potential values of [S2 j ], given that S1 9 = 2826 (as indicated by the red circle).
