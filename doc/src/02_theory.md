# Theory
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
