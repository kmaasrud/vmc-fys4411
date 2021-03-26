# Variational Monte Carlo solver

The solver is written in Rust (source code in `src`), and the statistical analysis and plotting is written in Python (source code in `python`). 

## Solver explanation

`wavefunction.rs`, `hamiltonian.rs` and `particle.rs` all contain a similarly named struct representing a respective part of the system. These are all tied together in the `System` struct located in `system.rs`. These represent the a system state and hold the equations to find relevant quantities.

Building on this, we've got the `Metropolis` trait located inside `metropolis.rs`, which describes an interface that is able to produce a Metropolis step. This trait is realised in the two structs `BruteForceMetropolis` and `ImportanceMetropolis`. Finally, located in `montecarlo.rs`, our Monte Carlo solver leverages this trait to perform an integration over the desired quantities.

The above are used to produce our results in the `produce_output.rs` file. Here we've defined a collection of functions that produce different types of outputs to suit our needs. Also worth mentioning is the `threadpool.rs` file, which contains a few handy functions to simplify parallelization.
