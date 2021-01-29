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

 $$\Psi_T(\mathbf r_1, ..., \mathbf r_N, \alpha, \beta) = \prod_i g(\alpha, \beta, \mathbf r_i) \prod_{j < k}f(a, |\mathbf r_j - \mathbf r_k|)$$ {#eq:trial-wavefunction}
 where


$$ g(\alpha,\beta,\mathbf{r}_i)= \exp{[-\alpha(x_i^2+y_i^2+\beta z_i^2)]}.$$ and

$$ f(a,|\mathbf{r}_i-\mathbf{r}_j|)=\Bigg\{
    \begin{array}{ll}
    0 & {|\mathbf{r}_i-\mathbf{r}_j|} \leq {a}\\
    (1-\frac{a}{|\mathbf{r}_i-\mathbf{r}_j|}) & {|\mathbf{r}_i-\mathbf{r}_j|} > {a}.
   \end{array} $$



 Once again, note the index notation explained in {@sec:index-notation-for-sums-and-products}.




## Importance sampling

<!-- For Theory:

- Explain difference between importance sampling and brute force sampling.
 For Practical:
- Run calculations for 1, 2 and 3 dim space, WITHOUT repulsive potential.
- Study the dependence of the results as a function of the time step $\delta$t
- Discuss(compare) results on difference between imp sampl and brute force metropolis. -->

Importance sampling, compared to the brute force Metropolis sampling, sets a bias on the sampling, leading it on a better path. This means that the desired standard deviation is acquired after fewer Monte Carlo cycles.

## Analytical
<!-- Rewrite  -->
As a test case to be compared against our numerical implementation, we want to find an analytical expression for the energy of the trial wave function(Ref)(local energy). We only study the harmonic oscillator potential and disregard the two-body potential. This is simply done by setting the parameter $a = 0$ which by {@eq:internal-potential} gives $V_\text{int} = 0$. First $\beta$ is set to 1 to find the relevant local energies for one to three dimensions for both one and N particles. The simplest Gaussian wavefunction then becomes:
<!-- Simple Gaussian Wavefunction  -->

$$\Psi_T(\mathbf{r_1, r_2,\ldots,r_N, \alpha, \beta}) = \prod_i \exp(-\alpha r_{i}^2).$$

The energy is here given by

$$
\begin{aligned}
E_L(\mathbf{r}) &=  \frac{1}{\Psi_T (\mathbf{r})} H \Psi_T (\mathbf{r})
= \frac{1}{\Psi_T (\mathbf{r})} \left[ \sum_i^N \left( \frac{-\hbar^2}{2m}
   \nabla_{i}^2 + V_{\text{ext}}({\mathbf{r}}_i)\right)  \right]\Psi_T(\mathbf{r}) \\
&= \frac{1}{\Psi_T(\mathbf{r})} \left[ \sum_i^N \left (\frac{-\hbar^2}{2m}
  \nabla_{i}^2 \Psi_T (\mathbf{r}) + V_\text{ext} ({\mathbf{r}}_i) \Psi_T(\mathbf{r}) \right) \right].
  \end{aligned}
$$

We simplify $\nabla_i^2\Psi_T$ as shown in {@sec:second-derivative-of-trial-wave-function} to get

$$\nabla^2\Psi_t(\mathbf r) = -2\alpha\Psi_T\left(\dim - 2\alpha\mathbf r_i^2\right),$$ {#eq:second-derivative-of-trial-wave-function}

where $\dim$ is the dimension of the system (1, 2 or 3). Given eq. {@eq:second-derivative-of-trial-wave-function}, we find that the local energy for the Gaussian wavefunction is

$$ E_L(\mathbf{r}) = \frac{\hbar^2 \alpha}{m} \left( \dim - {2  \alpha}\right) \left(1 + \frac{1}{2} m \omega^2_\text{ho}\right) \sum_i^N \mathbf{r}^2_{i},$$ {#eq:local-energy-gauss}

as shown in {@sec:local-energy-for-gaussian-wave-function}. We can simplify this even further by scaling, namely setting $\hbar = m = \omega_\text{ho}^2 = 1$, which gives us the equation

$$E_L(\mathbf{r}) = (\alpha \cdot \dim  - 3 \alpha^2) \sum_i^N \mathbf{r}^2_{i}$$ {#eq:local-energy-gauss-scaled}

### Drift force

The following expression for the drift force will be used to explanation

$$
F = \frac{2 \nabla_k \Psi_T(\mathbf{r})}{\Psi_T(\mathbf{r})} = -4 \alpha \mathbf{r}_{k}
$$

applying the gradient operator to the trail wavefunction is already shown in eq (REF). 
