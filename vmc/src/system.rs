use rand::prelude::random;
use crate::Particle;
use crate::WaveFunction;

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

    pub fn random_particle_change(&self, step_size: f64) -> Vec<Particle> {
        let mut new_particles = self.particles.clone();
        let i = random::<usize>() % self.particles.len();
        for d in 0..new_particles[i].dim {
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
