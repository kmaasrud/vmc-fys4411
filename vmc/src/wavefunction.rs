use crate::Particle;

pub trait WaveFunction {
    fn laplace(&self, particles: &Vec<Particle>) -> f64;
    fn gradient(&self, i_particle: usize) -> Vec<f64>;
    // Here I'm just going by what @Schoyen has done, let's see if it works
    fn drift_force(&self, i_particle: usize) -> Vec<f64> {
        self.gradient(i_particle).iter().map(|x| x * 2.).collect()
    }
}
