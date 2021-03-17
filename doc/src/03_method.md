# Method

Rust is used to preform the numerical calculations.
<!-- Maybe explain why we chose Rust instead of traditional C++? -->
## Variational Monte Carlo

Firstly the ground state energy of spherical harmonic oscillator $(\beta = 1)$ was calculated using Variational Monte Carlo with standard Metropolis sampling. To get the code up and running  we started with the simplest scenario in one dimension without any interaction.  Neutral units were implemented. Comparing the calculations from the analytic expression and the ones from the numerical calculated kinetic energy in order to compare CPU time for the two. We also compared CPU time for different number of atoms (N = 1 , 10 , 100 and 500).
Thereafter the same was done in two and three dimensions.

## Natrual length scale

1) Avoid underflow
2) Most quantum mechanical sytems have energy in units of $\hbar \omega$

Factor out $\hbar \omega_{ho}$ from the Hamiltonian

$$
H = \frac{1}{2} \sum_{i}^N{\left(- \frac{\hbar^2}{m}\ \nabla_i^2 + m\left[ \omega_{ho} (x_i^2 + y_i^2) + \omega_z^2 z_i^2\right] \right)}
$$

$$
= \frac{\hbar \omega_{ho}}{2} \sum_{i}^N{\left(- \frac{\hbar}{m \omega_{ho}} \nabla_i^2 + \frac{m \omega_{ho}}{\hbar}\left[ x_i^2 + y_i^2 + \frac{\omega_z^2}{\omega_{ho}^2} z_i^2\right] \right)}
$$

$\frac{\hbar \omega_{ho}}{2}$ has the unit of length, hence $\frac{\hbar \omega_{ho}}{2} = \sqrt{\text{length}}$, which is a natrual length scale. Defining 

$$
 a_{ho} = \sqrt{\frac{\hbar}{m \omega_{ho}}}, 
$$

and
$$
\gamma = \frac{\omega_z}{\omega_{ho}}
$$

Divide the Hamiltoninan with the factor of $a$ to be able to express the energy in units of $\hbar \omega$

$$
H = \frac{1}{2} \sum_{i}^N{\left(- a_{ho}^2\ \nabla_i^2 + a_{ho}^{-2} \left( x_i^2 + y_i^2 +\gamma^2 z_i^2\right) \right)}
$$

As for coordinates and particle(boson) diameter (scaling $x_i,y_i,z_i$ and the Laplacian) by doing as follows

$$
a_0  = \frac{a} {a_{ho}}, \quad
\mathbf{r_0} = \frac{\mathbf{r}} {a_{ho}}
$$

Scaled Hamiltonian 

$$
H = \frac{1}{2} \sum_{i}^N{\left(-\nabla_i^2 +  x_i^2 + y_i^2 +\gamma^2 z_i^2  \right)}
$$

Fixed parameters ( to only have one variational paramter): 
Boson diamter: $\frac{a}{a_{ho}} = ??$

 $\gamma = \beta = ...$