use crate::Particle;

pub struct System {
    particles: Vec<Particle>,
    dimensionality: u8,
}

impl System {
    pub fn new() -> System {
        System {
            particles: Vec::new(),
            dimensionality: 0,
        }
    }
}

pub struct HarmonicTrap  {
    pub mass: f64,
    pub omega_ho: f64,  //Trap frequency in the perpendicular xy plane
    pub omega_z: f64,   //Trap frequency in the z direction
}

impl HarmonicTrap {
    pub fn spherical(&mut self, r: f64) -> f64 { //Returns V_ext(r) for a spherical harmonic trap [currently 1D]
        0.5*self.mass*self.omega_ho*self.omega_ho*r*r
    }
    pub fn elliptical(&mut self, r: Vec<f64>) -> f64 { //Returns V_ext(r) for an elliptical harmonic trap
        0.5*self.mass*(self.omega_ho*self.omega_ho*(r[0]*r[0] + r[1]*r[1])+ self.omega_z*self.omega_z*r[2]*r[2])
    }
}
