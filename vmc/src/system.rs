use crate::Particle;

pub struct System {
    particles: Vec<Particle>,
    dimensionality: u8,
}

impl System {
    pub fn new() -> System {
        System{
            particles: Vec::new(),
            dimensionality: 0,
        }
    }
}