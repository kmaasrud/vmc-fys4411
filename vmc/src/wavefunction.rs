use crate::Particle;

pub trait WaveFunction {
    fn evaluate(&self, particles: &Vec<Particle>) -> f64;
    fn laplace(&self, particles: &Vec<Particle>) -> f64;
    fn gradient(&self, particle: Particle) -> Vec<f64>;
    fn drift_force(&self, particle: Particle) -> Vec<f64> {
        self.gradient(particle).iter().map(|x| x * 2.).collect()
    }
    fn quantum_force(&self, particle: &Particle, particles: &Vec<Particle>) -> Vec<f64>;
}


#[derive(Debug)]
pub struct GaussianWaveFunction {
    alpha: f64,
    beta: f64,
}

impl GaussianWaveFunction {
    pub fn new(alpha: f64) -> Self {
        GaussianWaveFunction { alpha: alpha, beta: alpha}
    }
}

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

    //  Takes input particle, find quantum force by evaluating its wavefunction
    fn quantum_force(&self, particle: &Particle, particles: &Vec<Particle>) -> Vec<f64> {
        // Loop over all other particles in order to calculate quantum force felt from all of them.
        // TODO: THIS SHOULD BE PARALELLIZED
        let mut r: f64;
        let mut deno: f64;
        let mut qforce: Vec<f64> = vec![0.; particle.dim];

        for other in particles {
            r = other.distance_to(particle);

            deno = 1. / (1. + self.beta * r);

            for i in 0..particle.dim {
                qforce[i] += -2. * particle.position[i] * self.alpha * (particle.position[i] - other.position[i]) * deno.powi(2) / r;
            }
        }
        // After this loop, all qforce vectors should be summed to create the total quantum force.
        // MORE: https://compphysics.github.io/ComputationalPhysics2/doc/pub/week4/html/week4-reveal.html slide 11
        qforce
    }
}
