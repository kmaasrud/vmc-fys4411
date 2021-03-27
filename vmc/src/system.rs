use crate::Particle;
use crate::WaveFunction;
use crate::Hamiltonian;
use rand::distributions::{Distribution, Uniform};
use rand::{prelude::random, thread_rng};
use rand_distr::Normal;

pub struct System {
    pub particles: Vec<Particle>,
    pub dimensionality: usize,
    pub wavefunction: WaveFunction,
    pub hamiltonian: Hamiltonian,
}

impl System {
    pub fn new(n_particles: usize, dim: usize, wavefunction: WaveFunction, hamiltonian: Hamiltonian) -> Self {
        System {
            particles: vec![Particle::new(dim); n_particles],
            dimensionality: dim,
            wavefunction: wavefunction,
            hamiltonian: hamiltonian,
        }
    }

    /// Creates a new system with particles distributed randomly
    pub fn distributed(n_particles: usize, dim: usize, wavefunction: WaveFunction, hamiltonian: Hamiltonian, spread: f64) -> Self {
        let mut rng = thread_rng();
        let uniform = Uniform::new(0., 1.);
        let mut sys: System = System::new(n_particles, dim, wavefunction, hamiltonian);
        let mut r: f64;

        for i in 0..sys.particles.len() {
            // Make a new randomly placed particle
            let mut new_particle: Particle = Particle::new(sys.dimensionality);
            new_particle.position = (0..dim).map(|_| spread * (uniform.sample(&mut rng) - 0.5)).collect();

            // Ensure it is not overlapping with other particles (this is an extra check in
            // addition to the pre monte carlo steps, not sure if we really need it)
            for other in sys.particles[..i].iter() {
                r = other.distance_to(&new_particle);
                while r < 0.0043 {
                    new_particle.position = (0..dim).map(|_| spread * (uniform.sample(&mut rng) - 0.5)).collect();
                    r = other.distance_to(&new_particle);
                }
            }
            sys.particles[i].position = new_particle.position;
        }
        sys
    }

    /// Change a random particle's position by a random value
    pub fn random_particle_change(&self, step_size: f64) -> Vec<Particle> {
        let mut new_particles = self.particles.clone();
        let i = random::<usize>() % self.particles.len();
        for d in 0..new_particles[i].dim {
            new_particles[i].position[d] += (random::<f64>() - 0.5) * step_size;
        }
        new_particles
    }

    /// Takes in a step size and returns the next particle state of the system.
    pub fn quantum_force_particle_change(&mut self, non_interacting: bool) -> (Vec<Particle>, usize) {
        let mut rng = thread_rng();
        let normal = Normal::new(0., 1.).unwrap();

        // 0.005 is hard-coded solution for delta t in Langevin equation
        let qf_step_size = 0.005;

        // Picks one random particle to do the change for
        let i = random::<usize>() % self.particles.len();

        self.particles[i].qforce = if non_interacting { self.wavefunction.quantum_force_non_interacting(&self.particles[i]) }
                                                 else { self.wavefunction.quantum_force(i, &self.particles) };

        // Clones the last particle state of the system
        let mut new_particles = self.particles.clone();
        // Loop over its dimensions and do Langevin equation
        for d in 0..new_particles[i].dim {
            new_particles[i].position[d] += 0.5 * self.particles[i].qforce[d] * qf_step_size
                + normal.sample(&mut rng) * qf_step_size.sqrt(); // 0.5 is the D constant.
        }

        // Calculate quantum force of new state
        new_particles[i].qforce = if non_interacting { self.wavefunction.quantum_force_non_interacting(&new_particles[i]) }
                                                else { self.wavefunction.quantum_force(i, &new_particles) };

        (new_particles, i)
    }
}
