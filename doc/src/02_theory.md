# Theory

The system in question is a hard sphere Bose gas located in a potential well. The potential is an *elliptical harmonic trap*, described for each particle by

$$V_\text{ext}(\mathbf r) = \frac{1}{2}m\left(\omega_\text{ho}^2(r_x^2 + r_y^2) + \omega_z^2 r_z^2\right).$$ {#eq:external-potential}

Here, $\mathbf r$ is the position of the particle and $\omega_\text{ho}$ is the frequency of the trap. Note that setting $\omega_\text{ho} = \omega_z$ results in eq. {@eq:external-potential} evaluating to $V_\text{ext}(\mathbf r) = \frac{1}{2}m\omega_\text{ho}^2r^2$, which represents the *spherical* case of the elliptical harmonic trap. As a simplification, we hereby denote the spherical case as (S) and the general elliptical case as (E).

In addition to this external potential, we represent the inter-boson interactions with the following pairwise, repulsive potential[@Project1]:

$$V_\text{int}(|\mathbf r_i - \mathbf r_j|) = \begin{cases}\infty & |\mathbf r_i - \mathbf r_j| \le a \\ 0 & |\mathbf r_i - \mathbf r_j| > a\end{cases},$$ {#eq:internal-potential}

where $a$ is the hard-core diameter of the bosons. Eq. {@eq:external-potential} and eq. {@eq:internal-potential} evaluate to the following two-body Hamiltonian:

$$H = \sum_i^N\left(-\frac{\hbar^2}{2m}\nabla_i^2 + V_\text{ext}(\mathbf r_i)\right) + \sum_{i < j}^N V_\text{int} (|\mathbf r_i - \mathbf r_j|).$$ {#eq:hamiltonian}

The term $-\frac{\hbar^2}{2m}\nabla_i^2$ stems from the kinetic energy of the system and the index notation used is described in {@sec:index-notation-for-sums-and-products}. By scaling into length units of $a_\text{ho}$ and energy units of $\hbar\omega_\text{ho}$, this equation is further simplified into:

$$ H = \frac{1}{2}\sum_i^N \left(-\nabla_i^2 + r_{x, i}^2 + r_{y, i}^2 + \gamma^2 r_{z, i}^2\right) + \sum_{i<j}^N V_\text{int}(|\mathbf r_i - \mathbf r_j|) ,$$ {#eq:scaled_ham}

where $\gamma = \frac{\omega_z}{\omega_\text{ho}}$. The derivation of {@eq:scaled_ham} is explained in {@sec:scaled_ham}. Lastly we also define the so-called local energy, which is the quantity we want to integrate over to find the total energy of the system:

$$ E_L(\mathbf r) = \frac{1}{\Psi_T(\mathbf r)}H\Psi_T(\mathbf r) $$ {#eq:local-energy}

## The variational principle

Given the above Hamiltonian, we can introduce the concept of a *trial wave function* $\Psi_T(\alpha)$. This is a normalized ansatz to the ground state wave function, parametrized by the parameter(s) $\alpha$. This gives us a way of deploying the *variational principle* by varying said parameter $\alpha$ to our needs:

We know that for any normalized function $\Psi_T$, the expected energy is higher than the ground state energy (as proved in [@Griffiths] on p. 293-294), viz.

$$ \langle E(\alpha) \rangle = \langle \Psi_T(\alpha) | H | \Psi_T(\alpha)\rangle \ge E_0 = \langle \Psi_0 | H | \Psi_0\rangle. $$ {#eq:variational-principle}

Thus, minimizing over $\alpha$ will give an approximation of the true ground state (perhaps even an accurate answer).

Evaluating this integral is computationally demanding. Hence, we utilize Monte Carlo integration to allow scalability. This is done by changing the particles positions where the shifting follows some rules. For each change, the local energy is sampled resulting in an expectation value of the energy $\langle E\rangle$ for the Hamiltonian.

To find the lowest value with regards to $\alpha$, we could either test over many different values, or use gradient descent methods. The latter requires an expression for $\frac{\partial E}{\partial \alpha}$, which we choose to define thusly:

$$ \dot E_\alpha = \frac{\partial \langle E_L(\alpha)\rangle}{\partial \alpha} .$$

Using the additional notation of $\dot \Psi_{T, \alpha} = \frac{\partial \langle \Psi_T(\alpha)\rangle}{\partial \alpha}$, it can be shown that by using the chain rule and the hermiticity of the Hamiltonian [@CompQuantum2020], we get the expression

$$ \dot E_\alpha = 2\left(\left\langle\frac{\dot \Psi_{T, \alpha}}{\Psi(\alpha)} E_L(\alpha)\right\rangle - \left\langle\frac{\dot \Psi_{T, \alpha}}{\Psi(\alpha)}\right\rangle \langle E_L(\alpha)\rangle\right) $$ {#eq:energy-deriv}

Further explanation on how this is used in our gradient descent method is explained in the section [Steepest gradient descent](#steepest-gradient-descent).

## Wave function

<!-- Here we need to either describe how we arrived at this, or cite Morten -->

For $N$ particles, we use the following trial wave function:

$$\Psi_T(\mathbf r_1, ..., \mathbf r_N, \alpha, \beta) = \prod_i g(\alpha, \beta, \mathbf r_i) \prod_{j < k}f(a, |\mathbf r_j - \mathbf r_k|)$$ {#eq:trial-wavefunction}

Once again, the index notation is described in {@sec:index-notation-for-sums-and-products}. Here we've used that

\begin{align*}
g(\alpha,\beta,\mathbf{r}_i) &= e^{-\alpha(x_i^2+y_i^2+\beta z_i^2)}, \\
\text{and }f(a,|\mathbf r_i-\mathbf r_j|) &= \begin{cases} 0 & |\mathbf r_i-\mathbf r_j| \le a \\ 1-\frac{a}{|\mathbf r_i-\mathbf r_j|} & {|\mathbf r_i-\mathbf r_j|} > a \end{cases},
\end{align*}

as shown in [@Project1]. Simplifying the trial wave function can prove useful, in order to reduce the number of floating point operations. An analytical expression is also convenient for comparison with the numerical calculations.


## Importance sampling

Importance sampling, compared to the brute force Metropolis sampling, sets a bias on the sampling, leading it on a better path. This means that the desired standard deviation is acquired after fewer Monte Carlo cycles.

For our quantum mechanical scenario with boson particles in a magnetic trap, the bias has its root in the so-called quantum force. This quantum force pushes the walker (the boson particle) to the regions where the trial wave function is large. It is clear that this yields a faster convergence compared to the Metropolis algorithm, where the walker has the same probability of moving in all directions.

The quantum force $\mathbf{F}$ is given by the formula

$$ \mathbf{F}=2 \frac{1}{\Psi_{T}} \nabla \Psi_{T}, $$

which is derived from the Fokker-Planck equation, using the Langevin equation to generate the next step with Euler's method, and by making the probability density converge to a stationary state.


### Fokker-Planck

For one particle (or walker), the one-dimensional Fokker-Planck equation for a diffusion process is:

$$ \frac{\partial P}{\partial t}=D \frac{\partial}{\partial x}\left(\frac{\partial}{\partial x}-F\right) P(x, t) $$

Where $P(x,t)$ is a time-dependent probability density, $D$ is the diffusion coefficient and $F$ is a drift term which in our case is driven by the quantum force.

<!-- @amundmr What is D and how can we find it? Later we set it to 1/2 in scaled units, but how do we know this. Probably sufficient to just reference someone else though. -->


### Langevin equation

The Langevin equation solution gives the position of the walker in the next timestep. The Langevin equation is:

$$ \frac{\partial x(t)}{\partial t}=D F(x(t))+\eta $$

Converting this to a function yielding the new position $y$ in a computational manner, we use Euler's method.

$$ y=x+D F(x) \Delta t+\xi \sqrt{\Delta t} .$$ {#eq:euler_method}

Here $x$ is the old position, $y$ is the new position and $\xi$ is a randomly sampled value from the normal distribution. In scaled units, the diffusion coefficient evaluates to $\frac{1}{2}$. The timestep $\Delta t$ has stable values within the range $\Delta t \in [0.001, 0.01]$ <!-- Citation necessary, @amundmr -->, so we'll simply choose the value $\Delta t = 0.005$ here.


### Fokker-Planck and Langevin equation in importance sampling

In order to use these equations for our importance sampling, we start with the original Fokker-Planck equation.

After inserting $D$ as the diffusion coefficient and $\mathbf{F}_{\mathbf{i}}$ as component $i$ of the drift velocity, we can make the probability density converge to a stationary state by setting its partial derivative over time to zero.

$$ \frac{\partial P}{\partial t}=\sum_{i} D \frac{\partial}{\partial \mathbf{x}_{\mathbf{i}}}\left(\frac{\partial}{\partial \mathbf{x}_{\mathbf{i}}}-\mathbf{F}_{\mathbf{i}}\right) P(\mathbf{x}, t) $$

Where then $\frac{\partial P}{\partial t}= 0$, and by expanding the parenthesis and moving the double partial derivative over to the other side, we obtain:

$$ \frac{\partial^{2} P}{\partial \mathbf{x}_{\mathbf{i}}^{2}}=P \frac{\partial}{\partial \mathbf{x}_{\mathbf{i}}} \mathbf{F}_{\mathbf{i}}+\mathbf{F}_{\mathbf{i}} \frac{\partial}{\partial \mathbf{x}_{\mathbf{i}}} P $$

By inserting $g(\mathbf{x}) \frac{\partial P}{\partial x}$ for the drift term, $\mathbf{F}$, we get

$$ \frac{\partial^{2} P}{\partial \mathbf{x} _{\mathbf{i}}{}^{2}}=P \frac{\partial g}{\partial P}\left(\frac{\partial P}{\partial \mathbf{x}_{i}}\right)^{2}+P g \frac{\partial^{2} P}{\partial \mathbf{x}_{i}^{2}}+g\left(\frac{\partial P}{\partial \mathbf{x}_{i}}\right)^{2} $$

Where again the left hand side can be set to zero to comply with the fact that at a stationary state, the probability density is the same for all walkers.

For this to be solvable, the remaining terms have to cancel each other. This is only possible when $g = P^{-1}$, which gives the aformentioned quantum force, $\mathbf{F},$

$$ \mathbf{F}=2 \frac{1}{\Psi_{T}} \nabla \Psi_{T}. $$

From here, The Green's function is deployed as

$$ G(y, x, \Delta t)=\frac{1}{(4 \pi D \Delta t)^{3 N / 2}} \exp \left(\frac{-(y-x-D \Delta t F(x))^{2}}{ 4 D \Delta t}\right) $$

Which will be part of the proposal distribution, $q(y,x)$ as

$$ q(y, x)=\frac{G(x, y, \Delta t)\left|\Psi_{T}(y)\right|^{2}}{G(y, x, \Delta t)\left|\Psi_{T}(x)\right|^{2}}$$ {#eq:proposal_distr}


## Analytical derivations

### Local energy simple Gaussian wave function

As a test case to be compared against our numerical implementation, we want to find an analytical expression for the energy of the trial wave function. We simplify by studying only the non-interacting part, which is done by setting the parameter $a = 0$. We also set $\beta = 1$, giving us the following trial wave function:

$$\Psi_T(\mathbf{r_1, r_2,\ldots,r_N, \alpha, \beta}) = \prod_i \exp(-\alpha r_{i}^2).$$

Considering {@eq:local-energy}:

\begin{align*}
E_L(\mathbf{r}) &=  \frac{1}{\Psi_T (\mathbf{r})} H \Psi_T (\mathbf{r})
= \frac{1}{\Psi_T (\mathbf{r})} \left[ \sum_i^N \left( \frac{-\hbar^2}{2m}
   \nabla_{i}^2 + V_{\text{ext}}({\mathbf{r}}_i)\right)  \right]\Psi_T(\mathbf{r}) \\
&= \frac{1}{\Psi_T(\mathbf{r})} \left[ \sum_i^N \left (\frac{-\hbar^2}{2m}
  \nabla_{i}^2 \Psi_T (\mathbf{r}) + V_\text{ext} ({\mathbf{r}}_i) \Psi_T(\mathbf{r}) \right) \right].
\end{align*}

We simplify $\nabla_i^2\Psi_T$ as shown in {@sec:second-derivative-of-trial-wave-function}, yielding

$$\nabla^2\Psi_t(\mathbf r) = -2\alpha\Psi_T\left(\dim - 2\alpha\mathbf r_i^2\right),$$ {#eq:second-derivative-of-trial-wave-function}

where $\dim$ is the dimension of the system (1, 2 or 3). Given eq. {@eq:second-derivative-of-trial-wave-function}, we find that the local energy for N particles in the case of the simple Gaussian wavefunction is

$$ E_L(\mathbf{r}) = \frac{\hbar^2 }{m} \alpha N \dim +  \left( \frac{1}{2} m \omega^2_\text{ho} - 2 \alpha^2\right)  \sum_i^N \mathbf{r}^2_{i},$$ {#eq:local-energy-gauss}

as shown in {@sec:local-energy-for-gaussian-wave-function}. We can simplify this even further by scaling, namely setting $\hbar = m = 1$, which gives us the equation

$$E_L(\mathbf{r}) = N\alpha  \dim  + \left(\frac{1}{2} m \omega^2_\text{ho} - 2 \alpha^2\right) \sum_i^N \mathbf{r}^2_{i}$$ {#eq:local-energy-gauss-scaled}

An even simpler analytic expression is obtained by setting $\omega_{\text{ho}} = 1$ and taking the derivate of the local energy with respect to $r_i$,  giving $\alpha= 0.5$.

$$E_L = \frac{N \dim}{2}$$


### Drift force

<!-- This is perhaps useful in another part of theory as well? Consider moving outside of Analytical derivations -->

The following expression for the drift force will be used to **explanation**

$$ F = \frac{2 \nabla_k \Psi_T(\mathbf{r})}{\Psi_T(\mathbf{r})} = -4 \alpha \mathbf{r}_{k} $$

applying the gradient operator to the trial wavefunction is already shown (appendix: Second derivative of trial wave function).

### Local energy for full wave function

With $\beta \ne 0$ and $\text{a} > 0$ the wave function becomes a bit more complicated as the potential/Gaussian can be  can now be elliptical  and the wave function contains the Jastrow factor. The energy is given as:

$$ E(\mathbf{r}) = \frac{1}{\Psi_T(\mathbf{r})}\sum_i^{N}\nabla_i^2\Psi_T(\mathbf{r}), $$

To simplify coming equations, we set $\phi(\mathbf r) = g(\alpha, \beta, \mathbf r)$, $u(r_{ij}) = \ln f(r_{ij})$ and $r_{ij} = |r_i - r_j|$. With eq. {@eq:trial-wavefunction}, this results in

$$\Psi_T(\mathbf{r})=\prod_i^N \phi(\mathbf{r}_i) \exp{\left(\sum_{i<j}u(r_{ij})\right)}$$

Using this simplification, we show in {@sec:trial_wf_gradient} that the gradient for the $k$-th particle is equal to:

\begin{align*} 
\nabla_k \Psi_T (\mathbf{r}) =\ &\nabla_k \phi (\mathbf{r}_ 4k)\left[\prod^N_{i \ne k}{\phi(\mathbf{r}_ k)} \right] \exp \left( \sum^N _{j<m} u(r _{jm})\right) \\ &+ \left[\prod^N _i\phi(\mathbf{r}_ i)\right] \exp \left( \sum^N _ {j<m} u(r _ {jm})\right) \sum^N _ {l\ne k } \nabla_ k (r_ {kl}).
\end{align*} 

Furthermore, using the resulting Laplacian found in {@sec:trial_wf_laplacian}, we can find

\begin{align*}
\frac{1}{\Psi_T(\mathbf{r})} \nabla_k^2 \Psi_T(\mathbf{r}) &= \frac{\nabla_k \phi(\mathbf{r}_k)}{\phi(\mathbf{r}_k)} + 2 \frac{\nabla_k \phi(\mathbf{r}_k)}{\phi(\mathbf{r}_k)}\sum _{j\ne k}
\frac{\mathbf{r}_j - \mathbf {r}_k}{\mathbf{r} _{jk}}u'(r _{lk})\\
&+ \sum _{j\ne k}\sum _{l\ne k}
\frac{\mathbf{r}_j - \mathbf {r}_k}{\mathbf{r} _{jk}} u'(r _{lk}) \\
&+ \sum _{j\ne k}\sum _{l\ne k}
\frac{\mathbf{r}_j - \mathbf {r}_k}{\mathbf{r} _{jk}} \frac{\mathbf{r}_l - \mathbf {r}_k}{\mathbf{r} _{lk}}  u'(r _{jk})  u'(r _{lk}) \\
&+ \sum _{l\ne k} \frac{2}{r _{lk}} u'(r _{lk}) +  u''(r _{lk})
\end{align*}

where these hold:

\begin{align*}
\frac{\nabla_k \phi(\mathbf{r}_k)}{\phi(\mathbf{r}_k)} &= -2\alpha \left[
\begin{matrix}
x_k^2 \\ y_k^2 \\ \beta z_k^2
\end{matrix}\right], \\ \\
\frac{\nabla_k^2 \phi(\mathbf{r}_k)}{\phi(\mathbf{r}_k)} &= 2\alpha (2\alpha)[x_k^2 + y_k^2 + \beta^2z_k^2] - 2 - \beta),\\ \\
u'(r_{ij}) &= \frac{r_{ij}}{r_{ij}-a}, \quad \text{for}  \quad r_{ij}  > a, \\ \\
u''(r_{ij}) &= \frac{a(a-2r_{ij})}{r_{ij}^2(a-r_{ij})^2}, \quad \text{for} \quad r_{ij}  > a.
\end{align*}

<!-- Are you done here, @annasro? -->
