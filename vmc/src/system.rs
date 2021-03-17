use crate::Particle;
use crate::WaveFunction;
use crate::Hamiltonian;
use rand::distributions::{Distribution, Uniform};
use rand::{prelude::random, thread_rng};

#[derive(Debug)]
pub struct System<T: WaveFunction, V: Hamiltonian> {
    pub particles: Vec<Particle>,
    pub dimensionality: usize,
    pub wavefunction: T,
    pub hamiltonian: V,
}

impl<T, V> System<T, V> where T: WaveFunction, V: Hamiltonian {
    pub fn new(n_particles: usize, dim: usize, wavefunction: T, hamiltonian: V) -> Self {
        System {
            particles: vec![Particle::new(dim); n_particles],
            dimensionality: dim,
            wavefunction: wavefunction,
            hamiltonian: hamiltonian,
        }
    }

    pub fn distributed(n_particles: usize, dim: usize, wavefunction: T, hamiltonian: V, spread: f64) -> Self {
        let mut rng = thread_rng();
        let uniform = Uniform::new(-1., 1.);
        let mut sys: System<T, V> = System::new(n_particles, dim, wavefunction, hamiltonian);

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
    pub fn quantum_force_particle_change(&mut self, step_size: f64 ) -> (Vec<Particle>, usize) {
        // This func is called from metropolis.rs:ImportanceMetropolis in order to
        // do one step (change the particle system) WITH SAMPLING BIAS: the quantum force.
        let mut rng = thread_rng();
        let uniform = Uniform::new(-1., 1.);

        // Picks one random particle to do the change for
        let i = random::<usize>() % self.particles.len();
        // Clones the last particle state of the system
        let mut new_particles = self.particles.clone();

        let quantum_force: Vec<f64> = self.wavefunction.quantum_force(&new_particles[i], &new_particles);
        self.particles[i].qforce = self.wavefunction.quantum_force(&self.particles[i], &self.particles);

        for d in 0..new_particles[i].dim {
            // Loop over its dimensions and do Langevin equation
            new_particles[i].position[d] += 0.5 * quantum_force[d] * step_size
                    + uniform.sample(&mut rng) * step_size.sqrt(); // 0.5 is the D constant.
        }
        new_particles[i].qforce = quantum_force;
        (new_particles, i)
    }
}

// pub struct HarmonicTrap  {
//     pub mass: f64,
//     pub omega_ho: f64,  //Trap frequency in the perpendicular xy plane
//     pub omega_z: f64,   //Trap frequency in the z direction
// }

// impl HarmonicTrap {
//     pub fn spherical(&mut self, r: f64) -> f64 { //Returns V_ext(r) for a spherical harmonic trap [currently 1D]
//         0.5*self.mass*self.omega_ho*self.omega_ho*r*r
//     }
//     pub fn elliptical(&mut self, r: Vec<f64>) -> f64 { //Returns V_ext(r) for an elliptical harmonic trap
//         0.5*self.mass*(self.omega_ho*self.omega_ho*(r[0]*r[0] + r[1]*r[1])+ self.omega_z*self.omega_z*r[2]*r[2])
//     }
// }
