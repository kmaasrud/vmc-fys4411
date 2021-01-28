# Theory

## The system described

<!-- We should describe the task at hand in the Introduction, but here we can express the details of the system in question. -->

The system in question is a hard sphere Bose gas<!-- https://www.kmaasrud.com/brain/bose-gas -->. The system is affected by an external potential - an *elliptical harmonic trap* - that is described by the following equation:

$$V_\text{ext}(\mathbf r) = \frac{1}{2}m\left(\omega_\text{ho}^2(x^2 + y^2) + \omega_z z^2\right).$$ {#eq:external-potential}

Note that setting $\omega_\text{ho} = \omega_z$ results in eq. {@eq:external-potential} evaluating to $V_\text{ext} = \frac{1}{2}m\omega_\text{ho}^2r^2$, which represents the *spherical* case of the elliptical harmonic trap. In addition to this external potential, we represent the inter-boson interactions with the following pairwise, repulsive potential:

$$V_\text{int}(|\mathbf r_i - \mathbf r_j|) = \begin{cases}\infty & |\mathbf r_i - \mathbf r_j| \le a \\ 0 & |\mathbf r_i - \mathbf r_j| > a\end{cases}.$$ {#eq:internal-potential}

Eq. {@eq:external-potential} and eq. {@eq:internal-potential} evaluate to the following two-body Hamiltonian:

$$H = \sum_i^N\left(-\frac{\hbar^2}{2m}\nabla_i^2 + V_\text{ext}(\mathbf r_i)\right) + \sum_{i < j}^N V_\text{int} (|\mathbf r_i - \mathbf r_j|).$$

The index notation used here is described in {@sec:index-notation-for-sums-and-products}.

## Wave function of the system

<!-- Some motivation for using the trial wave function is needed here. I've just written the following as a placeholder for now. -->

For a system of $N$ particles, we use the following trial wave function:

$$\Psi_T(\mathbf r_1, ..., \mathbf r_N, \alpha, \beta) = \prod_i g(\alpha, \beta, \mathbf r_i) \prod_{j < k}f(a, |\mathbf r_j - \mathbf r_k|). $$ {#eq:trial-wavefunction}

Once again, note the index notation explained in {@sec:index-notation-for-sums-and-products}.


## Analytical

<!-- rewrite!! Copied from project description -->
Want to use the Variational Monte Carlo (VMC) method to evaluate the ground state energy of a trapped, hard sphere Bose gas for different numbers of particles with a specific trail wavefunction (from the Variational method). Implement a trailfunction in a spherical or elliptical harmonic trap in 1-3 dimensions.
- Something about the wavefunction/ SE.
A way of solving the many body problem is to introduce the Schrödinger equation.  
In order to solve the many body problem
- Why we want to use a trail wavefunction.
- Explain why an analytical expression reduces potential no. of FLOPS.
1. The wavefunction(trail)with N atoms


$$\Psi_T(\mathbf{r})=\Psi_T(\mathbf{r}_1, \mathbf{r}_2, \dots \mathbf{r}_N,\alpha,\beta)
 =\left[
    \prod_i g(\alpha,\beta,\mathbf{r}_i)
 \right]
 \left[
    \prod_{j<k}f(a,|\mathbf{r}_j-\mathbf{r}_k|)
 \right], $$

3. Local energy
