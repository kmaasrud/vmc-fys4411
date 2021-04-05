# Discussion

## Numerical solver compared to analytical solutions {-}
### Local energy {-}
The local energy for different alphas are shown in the figures{@fig:BF_vs_IM_VS_analytical_1D}, {@fig:BF_vs_IM_VS_analytical_2D} and {@fig:BF_vs_IM_VS_analytical_3D}. The plots originates from caluclations utilizing the Brute Force Metropolis, Importance Sampling and analytically for the three dimensions and different number of particles. 

All the systems has one common point, namely $\alpha = 0.5$. The two numerical methods have, for most systems, an energy minima at this point, which is expected. 

Increasing the size of the sytem (higher number of dimensions and particles) the energy approaches a more linear dependency of alpha.  The Importance Sampling method is aproaching linearity faster than the Brute Force method. A possible explenation is decreasing accuracy for an increasing size of the system, because the shape of the energy as a function of alpha is expected to be a parabola. However, as can be seen, the two numerical methods are also approaching the analytical values for the energy for an increased size of the system. 

Overall the analytical calulated energy is approximatly linear. However, the expresson for the local energy (eq. {@eq:local-energy-gauss-scaled}) is proportional to $\alpha^2$. It is expected to be a parobola with an energy minima at $\alpha = 0.5$. Guessing there is something wrong with setup of the particle positions. 



### Performance {-}
To measue computational efficency of the Brute Force Metropolis and Importance Sampling methods, time for calculating energy for a specific alpha (here $\alpha = 0.5$) was measured. The times are listed in Table {@tbl:BF_vs_IM_VS_analytical}. It should be kept in mind that not all the time used for the calculations , especially for the analytic and low dimensions and few particle measurments as they are quite fast. In the mentioned cases, writing to files, may be a small, but not insignificant part of the measurment. 
    
Hence looking at a more overall picture, as expected, the times increases proportionaly with the number of dimensions and particles.  The Brute Force Metropolis method is for all the systems studied faster than the Importance sampling method. For the larger systems $D = 2,3$ and $N = 10 - 500$ the speedup is approximately 20 - 25 % for the Brute Force Metropolis algorithm. 

The results from the analytical measurment is another story. As can be seen, it is of orders of magnitude faster that the two numerical methods. In the columns where the time is $0.0$s the computation was too fast to even get a measurment. As the systems were calculated using different setups/algorithms and because of a high inaccuracy in the measurment, no further conclusions will be drawn from this result. However, it is important to point out the fact in having analytical expression for the local energy, will give a significant calculation speedup. It is the exception rather than the rule, having the analytical expression. 

## Brute force or importance sampling {-}

In our testing, the brute force Metropolis algorithm generally produced better results, especially in terms of convergence, when employing steepest gradient descent. This was surprising, and not in tune with our expectations. The cause here is probably a poor implementation. We reach the desired results by using importance sampling, but it's apparent that we loose some accuracy throughout the algoritm. For further improving this solver, this is the first issue that needs to be tackled, in order to minimize the amount of Monte Carlo cycles needed to produce a confident result.

## Convergence of SGD {-}

The convergence from different alphas to the correct one ($\alpha = 0.5$) in figure \ref{fig:sgd-alphas} shows that the convergence happens faster for $\alpha$-values below the correct one, while higher $\alpha$-values converges slower. The reason for this is that the derivative of the energy has a larger value when $\alpha < 0.5$, which by equation {@eq:sgd} shows that the step size is larger, in turn yielding faster convergence.

This being said, our SGD converged correctly and regardless of the starting value of $\alpha$, and thus works as expected.

## General performance {-}

In general, the variational Monte Carlo solver yielded the desired results confidently. By using Metropolis Monte Carlo integration together with steepest gradient descent, we were able to produce the correct optimal value of $\alpha$ for both a non-interacting system and an interacting one - with differing number of particles and dimensionality.
