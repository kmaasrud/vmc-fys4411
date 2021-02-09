use crate::System;

trait Hamiltonian {
    pub fn kinetic_energy(system: System) -> f64 {
        - system.omega / 2. * system.wavefunction.laplacian(system.particles)
    }
}