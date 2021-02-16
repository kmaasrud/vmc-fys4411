# Method
Rust is used to preform the numerical calculations.
<!-- Maybe explain why we chose Rust instead of traditional C++? -->
## Variational Monte Carlo
Firstly the ground state energy of spherical harmonic oscillator $(\beta = 1)$ was calculated using Variational Monte Carlo with standard Metropolis sampling. To get the code up and running  we started with the simplest scenario in one dimension without any interaction.  Neutral units were implemented. Comparing the calculations from the analytic expression and the ones from the numerical calculated kinetic energy in order to compare CPU time for the two. We also compared CPU time for different number of atoms (N = 1 , 10 , 100 and 500).
Thereafter the same was done in two and three dimensions. 
