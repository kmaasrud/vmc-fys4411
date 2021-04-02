# Discussion

## Numerical solver compared to analytical solutions

## *Brute force* or *importance sampling*

In our testing, the brute force Metropolis algorithm generally produced better results, especially in terms of convergence when employing steepest gradient descent. This was surprising, and not in tune with our expectations. The cause here is probably a poor implementation. We reach the desired results by using importance sampling, but it's apparent that we loose some accuracy throughout the algoritm. For further improving this solver, this is the first issue that needs to be tackled, in order to minimize the amount of Monte Carlo cycles needed to produce a confident result.

## Convergence of SGD

## General performance

In general, the variational Monte Carlo solver yielded the desired results confidently. By using Metropolis Monte Carlo integration together with steepest gradient descent, we were able to produce the correct optimal value of $\alpha$ for both a non-interacting system and an interacting one - with differing number of particles and dimensionality.
