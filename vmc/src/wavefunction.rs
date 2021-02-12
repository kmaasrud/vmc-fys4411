use crate::Particle;

pub trait WaveFunction {
    fn evaluate(&self, particles: &Vec<Particle>) -> f64;
    fn laplace(&self, particles: &Vec<Particle>) -> f64;
    fn gradient(&self, particle: Particle) -> Vec<f64>;
    // Here I'm just going by what @Schoyen has done, let's see if it works
    fn drift_force(&self, particle: Particle) -> Vec<f64> {
        self.gradient(particle).iter().map(|x| x * 2.).collect()
    }
}


// TODO: Docstring
pub struct GaussianWaveFunction {
    alpha: f64,
}

impl GaussianWaveFunction {
    fn new(alpha: f64) -> Self {
        GaussianWaveFunction { alpha: alpha, }
    }
}

// TODO: Check all these impls with our own theory. Just going by @mhjensen and @Schoyens word here.
impl WaveFunction for GaussianWaveFunction {
    fn evaluate(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        (-self.alpha * squared_position_sum).exp()
    }

    fn laplace(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        let dim = particles[0].dim as f64;
        let n = particles.len() as f64;

        2. * dim * n * self.alpha + 4. * squared_position_sum * self.alpha.powi(2)
    }

    fn gradient(&self, particle: Particle) -> Vec<f64> {
        particle.position.iter().map(|x| -2. * self.alpha * x).collect()
    }
}
