use rand::{prelude::random, thread_rng};
use rand::distributions::{Uniform, Distribution};
use crate::Particle;
use crate::WaveFunction;

#[derive(Debug)]
pub struct System<T: WaveFunction> {
    pub particles: Vec<Particle>,
    pub dimensionality: usize,
    pub wavefunction: T
}

impl<T> System<T> where T: WaveFunction {
    pub fn new(n_particles: usize, dim: usize, wavefunction: T) -> Self {
        System {
            particles: vec![Particle::new(dim); n_particles],
            dimensionality: dim,
            wavefunction: wavefunction,
        }
    }

    pub fn distributed(n_particles: usize, dim: usize, wavefunction: T, spread: f64) -> Self {
        let mut rng = thread_rng();
        let uniform = Uniform::new(-1., 1.);
        let mut sys: System<T> = System::new(n_particles, dim, wavefunction);

        for i in 0..sys.particles.len() {
            // Overlapping particles could be a problem...
            let mut particle = Particle::new(dim);
            particle.position = particle.position.iter().map(|x| x + spread * uniform.sample(&mut rng)).collect();
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

    //  This func is called from metropolis.rs:ImportanceMetropolis in order to 
    //  do one step (change the particle system) WITH SAMPLING BIAS: the quantum force.
    pub fn quantum_force_particle_change(&self, step_size: f64) -> Vec<Particle>{ // Takes self (the particle system), and a step size, returns vector with particle system at next eventual step.
        let mut new_particles = self.particles.clone();     // Clones last particle step in order to change it
        let i = random::<usize>() % self.particles.len();   // Picks one random particle to do the change for
        for d in 0..new_particles[i].dim {                  // Loop over its dimensions and change each of it a random value between -1 and 1 multiplied by stepsize.
            new_particles[i].position[d] += 2. * (random::<f64>() - 0.5) * step_size;
        }
        new_particles
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
