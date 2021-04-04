# Discussion

## Numerical solver compared to analytical solutions {-}

## Brute force or importance sampling {-}

In our testing, the brute force Metropolis algorithm generally produced better results, especially in terms of convergence, when employing steepest gradient descent. This was surprising, and not in tune with our expectations. The cause here is probably a poor implementation. We reach the desired results by using importance sampling, but it's apparent that we loose some accuracy throughout the algoritm. For further improving this solver, this is the first issue that needs to be tackled, in order to minimize the amount of Monte Carlo cycles needed to produce a confident result.

## Convergence of SGD {-}

The convergence from different alphas to the correct one ($\alpha = 0.5$) in figure \ref{fig:sgd-alphas} shows that the convergence happens faster for $\alpha$-values below the correct one, while higher $\alpha$-values converges slower. The reason for this is that the derivative of the energy has a larger value when $\alpha < 0.5$, which by equation {@eq:sgd} shows that the step size is larger, in turn yielding faster convergence.

This being said, our SGD converged correctly and regardless of the starting value of $\alpha$, and thus works as expected.

## General performance {-}

In general, the variational Monte Carlo solver yielded the desired results confidently. By using Metropolis Monte Carlo integration together with steepest gradient descent, we were able to produce the correct optimal value of $\alpha$ for both a non-interacting system and an interacting one - with differing number of particles and dimensionality.
