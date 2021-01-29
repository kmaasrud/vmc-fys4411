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

> For Theory:
> - Explain difference between importance sampling and brute force sampling.
> For Practical:
> - Run calculations for 1, 2 and 3 dim space, WITHOUT repulsive potential.
> - Study the dependence of the results as a function of the time step $\delta$t
> - Discuss(compare) results on difference between imp sampl and brute force metropolis.


Importance sampling, compared to the brute force Metropolis sampling, sets a bias on the sampling, leading it on a better path. This means that the desired standard deviation is acquired after fewer Monte Carlo cycles.


## Analytical
<!-- Rewrite  -->
For reasons to be pointed out, there is an advantage of finding an analytical expression for the energy of the trail wavefunction(Ref)(local energy). Only studying the case of the Harmonic oscillator potential, discarding the two body potential, by setting the parameter $a = 0$ in (REF: f(...)).  First $\beta$ is set to 1 to find the relevant local energies for one to three dimensions for both one and N particles.

The simplest Gaussian wavefunction then becomes
<!-- Simple Gaussian Wavefunction  -->

$$\Psi_T(\mathbf{r_1, r_2,\ldots,r_N, \alpha, \beta}) = \prod_i \exp(-\alpha r_{i}^2) $$

For the simplest case, the energy of the Gaussian Wave Function, is given by

$$
\begin{aligned}
E_L(\mathbf{r}) &=  \frac{1}{\Psi_T (\mathbf{r})} H \Psi_T (\mathbf{r})
= \frac{1}{\Psi_T (\mathbf{r})} \left[ \sum_i^N \left( \frac{-\hbar^2}{2m}
   \nabla_{i}^2 + V_{ext}({\mathbf{r}}_i)\right)  \right]\Psi_T(\mathbf{r}) \\
&= \frac{1}{\Psi_T(\mathbf{r})} \left[ \sum_i^N \left (\frac{-\hbar^2}{2m}
  \nabla_{i}^2 \Psi_T (\mathbf{r}) + V_{ext} ({\mathbf{r}}_i) \Psi_T(\mathbf{r}) \right) \right]
  \end{aligned}
$$


Intermediate calculation(mellomregning)
<!-- Anna don't understand the last step -->
$$
\begin{aligned}
\nabla_{i}^2 \Psi_{T}(\mathbf{r})
&= \nabla_{i} \cdot\left[\frac{\partial}{\partial x_{i}}, \frac{\partial}{\partial y_{i}},   
   \frac{\partial}{\partial z_{i}}\right] \Psi_{T}(\mathbf{r}) \\
&= \nabla_i \cdot \left[\frac{\partial}{\partial x_i} \exp{(-\alpha
   \mathbf{r}_i^2}),\frac{\partial}{\partial y_i} \exp{(-\alpha \mathbf{r}_i^2}), \frac{\partial}{\partial z_i} \exp{(-\alpha \mathbf{r}_i^2})\right] \\
&= \nabla_{i} \cdot \left[-2 \alpha x_{i} \exp{(-\alpha \mathbf{r}_{i}^{2}}), -2 \alpha
   y_{i}  
   \exp{(-\alpha \mathbf{r}^2_{i}}), -2 \alpha z_{i} \exp{(-\alpha \mathbf{r}_{i}^2})
   \right] \\
&= -2 \alpha \left[  \exp{(-\alpha \mathbf{r}^2_{i}})(1 - 2 \alpha x^2_{i}), \exp{(-\alpha
   \mathbf{r}^2_{i}})(1 - 2 \alpha y^2_{i}), \exp{(-\alpha \mathbf{r}^2_{i}})
   (1 - 2 \alpha z^2_{i}) \right] \\
&= -2\alpha \Psi_{T} \left[(1 - 2 \alpha x^2_{i}), (1 - 2 \alpha y^2_{i}),
   (1 - 2 \alpha  z^2_{i}) \right]\\
&= -2\alpha \Psi_{T}\sum_{d = x,y,z}1 -2\alpha d_{i}^2 \\
&= -2\alpha \Psi_{T}(dim - 2 \alpha  \mathbf{r}^2_{i})
\end{aligned}
$$

where $dim$ is the dimension of the system (1, 2 or 3)
The local energy, $E_L$, for the Gaussian wavefunction is then
$$
\begin{aligned}
E_L(\mathbf{r}) &=
    \frac{1}{\Psi_T(\mathbf{r})} \left[ \sum_i^N \left (\frac{-\hbar^2}{2m}
    \nabla_{i}^2 \Psi_T (\mathbf{r}) + V_{ext} ({\mathbf{r}}_i) \Psi_T(\mathbf{r}) \right)  
    \right]\\
&=  \frac{1}{\Psi_T(\mathbf{r})}  \left[ \sum_i^N \left(  \frac{\hbar^2 \alpha}{m}  (dim - 2
    \alpha  \mathbf{r}^2_{i} ) + \frac{1}{2} m \omega^2_{ho} \mathbf{r}^2_{i} \right) \Psi_T(\mathbf{r}) \right ] \\
&=  \sum_i^N \left( \frac{\hbar^2 \alpha}{m}  (dim - 2
    \alpha  \mathbf{r}^2_{i} ) + \frac{1}{2} m \omega^2_{ho} \mathbf{r}^2_{i} \right)\\
&=  \frac{\hbar^2 \alpha}{m} \left( dim - {2  \alpha}\right) (1 +   \frac{1}{2} m \omega^2_{ho} ) \sum_i^N \mathbf{r}^2_{i}  
\end{aligned}
$$

Simplifying further by setting $\hbar = m = 1$

$$
E_L(\mathbf{r}) =
\alpha(dim - 2\alpha)(1+ \frac{1}{2}\omega^2_{ho}) \sum_i^N \mathbf{r}^2_{i}  
$$

For $\omega^2_{ho} = 1$ it simplifies even further

$$
E_L(\mathbf{r}) =
(\alpha \cdot dim  - 3 \alpha^2) \sum_i^N \mathbf{r}^2_{i}  
$$

###Driftforce

The following expression for the drift force will be used to explanation

$$
F = \frac{2 \nabla_k \Psi_T(\mathbf{r})}{\Psi_T(\mathbf{r})} = -4 \alpha \mathbf{r}_{k}
$$

applying the gradient operator to the trail wavefunction is already shown in eq (REF). 
