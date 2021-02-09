use crate::Particle;

pub trait WaveFunction {
    fn new() -> Self;
    fn laplace(&self, particles: &Vec<Particle>) -> f64;
}