use crate::{WaveFunction, Particle};


#[derive(Clone)]
pub struct Hamiltonian {
    gamma_squared: f64,
}

impl Hamiltonian {
    // --- Constructors ---
    pub fn spherical() -> Self {
        Hamiltonian { gamma_squared: 1. }
    }
    pub fn elliptical(gamma: f64) -> Self {
        Hamiltonian { gamma_squared: gamma.powi(2) }
    }

    // --- Kinetic energies ---
    /// Kinetic energy for non-interacting particles using numerically calculated Laplacian
    fn kinetic_non_interacting(&self, wf: &WaveFunction, particles: &mut Vec<Particle>) -> f64 {
        - 0.5 * wf.laplace(particles, true)
    }
    /// Kinetic energy for interacting particles using numerically calculated Laplacian
    fn kinetic(&self, wf: &WaveFunction, particles: &mut Vec<Particle>) -> f64 {
        - 0.5 * wf.laplace(particles, false)
    }

    // --- Potential energy ---
    /// Potential energy of the spherical/elliptical harmonic trap
    fn trap_potential(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum_scaled_z(&self.gamma_squared)).sum();
        0.5 * squared_position_sum
    }

    // --- Total local energies ---
    /// Total local energy, assuming non-interacting particles
    pub fn energy_non_interacting(&self, wf: &WaveFunction, particles: &mut Vec<Particle>) -> f64 {
        self.kinetic_non_interacting(wf, particles) + self.trap_potential(particles)
    }
    /// Total local energy for interacting particles
    pub fn energy(&self, wf: &WaveFunction, particles: &mut Vec<Particle>) -> f64 {
        self.kinetic(wf, particles) + self.trap_potential(particles)
    }
}
