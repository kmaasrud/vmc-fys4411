use crate::Particle;
use crate::WaveFunction;
use crate::Hamiltonian;
use rand::distributions::{Distribution, Uniform};
use rand::{prelude::random, thread_rng};

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

    pub fn distributed(n_particles: usize, dim: usize, wavefunction: WaveFunction, hamiltonian: Hamiltonian, spread: f64) -> Self {
        let mut rng = thread_rng();
        let uniform = Uniform::new(-1., 1.);
        let mut sys: System = System::new(n_particles, dim, wavefunction, hamiltonian);

        for i in 0..sys.particles.len() {
            // Overlapping particles could be a problem...
            let mut particle = Particle::new(dim);
            particle.position = particle.position
                .iter()
                .map(|x| x + spread * uniform.sample(&mut rng))
                .collect();
            sys.particles[i] = particle;
        }

        sys
    }

    pub fn random_particle_change(&self, step_size: f64) -> Vec<Particle> {
        let mut new_particles = self.particles.clone();
        let i = random::<usize>() % self.particles.len();
        for d in 0..new_particles[i].dim {
            new_particles[i].position[d] += 2. * (random::<f64>() - 0.5) * step_size;
        }
        new_particles
    }

    /// Takes in a step size and returns the next particle state of the system.
    pub fn quantum_force_particle_change(&mut self) -> (Vec<Particle>, usize) {
        let mut rng = thread_rng();
        let uniform = Uniform::new(-1., 1.);

        // 0.005 is hard-coded solution for delta t in Langevin equation
        let qf_step_size = 0.005;

        // Picks one random particle to do the change for
        let i = random::<usize>() % self.particles.len();

        // Clones the last particle state of the system
        let mut new_particles = self.particles.clone();

        new_particles[i].qforce = self.wavefunction.quantum_force(i, &new_particles);
        self.particles[i].qforce = self.wavefunction.quantum_force(i, &self.particles);

        // Loop over its dimensions and do Langevin equation
        for d in 0..new_particles[i].dim {
            new_particles[i].position[d] += 0.5 * new_particles[i].qforce[d] * qf_step_size + uniform.sample(&mut rng) * qf_step_size.sqrt(); // 0.5 is the D constant.
        }

        (new_particles, i)
    }
}
