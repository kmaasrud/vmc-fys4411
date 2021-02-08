use crate::Particle;
use crate::WaveFunction;

pub struct System<T: WaveFunction> {
    particles: Vec<Particle>,
    dimensionality: usize,
    wavefunction: T
}

impl<T> System<T> where T: WaveFunction {
    pub fn new(n_particles: usize, dim: usize, wavefunction: T) -> Self {
        System {
            particles: vec![Particle::new(dim); n_particles],
            dimensionality: dim,
            wavefunction: wavefunction,
        }
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