# Method

## Importance sampling
With importance sampling, the walk in the coordinate space will be biased by the trail wave function.(hvis jeg bare forsto hva jeg skreiv nå..) We will base our approach on the Fokker-Planck and Langevin equations, which is used for trajectory generation in the coordinate space. (Er dette for mye kopiert fra [Importance Sampling](http://compphysics.github.io/ComputationalPhysics2/doc/pub/week3/html/week3.html)?)

For one particle (sometimes also called walker), the **one-dimensional** Fokker-Planck equation for a diffusion process is:
$$
\frac{\partial P}{\partial t}=D \frac{\partial}{\partial x}\left(\frac{\partial}{\partial x}-F\right) P(x, t)
$$
Where $P(x,t)$ is is a time-dependent probability density, $D$ is the diffusion coefficient and $F$ is a drift term.

From here, the Langevin equation is solved using Euler's method, which will give us the new positions in coordinate space.
$$
\frac{\partial x(t)}{\partial t}=D F(x(t))+\eta
$$
Where $\eta$ is a random variable. This gives the new position:
$$
y=x+D F(x) \Delta t+\xi \sqrt{\Delta t}
$$
Where the letters and symbols mean:
|Variable|Description|
|---|---|
|y | new position|
|x |old position
|DF(x) | Diffusion and Drift at the old possition|
|D | In AU*: 1/2, from the kinetic energy operator|
|$\Delta$t | Chosen time-step|
|$\xi$ | Gaussian random variable |
\* Atomic Units
Examples of timesteps giving stable values of the ground state energy is $\Delta t \in[0.001,0.01]$

## Fokker-Planck
The Fokker-Planck equation is an approximation of the time-dependent probability density $P(x,t)$ which describes isotropic diffusion of the particle.

$$
\frac{\partial P}{\partial t}=\sum_{i} D \frac{\partial}{\partial \mathbf{x}_{\mathbf{i}}}\left(\frac{\partial}{\partial \mathbf{x}_{\mathbf{i}}}-\mathbf{F}_{\mathbf{i}}\right) P(\mathbf{x}, t)
$$

$D$ is the diffusion coefficient, and $\mathbf{F}_{\mathbf{i}}$ is the drift velocity of component $i$. The cause of the drift velocity is an external potential, which for us is the magnetic field used to hold the boson gas in place.