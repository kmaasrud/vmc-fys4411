use crate::WaveFunction;
use crate::Particle;

pub trait Hamiltonian {
    fn kinetic<T: WaveFunction>(&self, wf: &T, particles: &Vec<Particle>) -> f64;
    fn potential(&self, particles: &Vec<Particle>) -> f64;
    fn local_energy<T: WaveFunction>(&self, wf: &T, particles: &Vec<Particle>) -> f64 {
        self.kinetic(wf, particles) + self.potential(particles)
    }
}


#[derive(Debug)]
pub struct HarmonicOscillator {
    lambda: f64,
    omega: f64,
}

impl HarmonicOscillator {
    pub fn spherical(omega: f64) -> Self {
        HarmonicOscillator { lambda: 1., omega: omega }
    }
    pub fn elliptical(lambda: f64, omega: f64) -> Self {
        HarmonicOscillator { lambda: lambda, omega: omega }
    }
}

impl Hamiltonian for HarmonicOscillator {
    fn kinetic<T: WaveFunction>(&self, wf: &T, particles: &Vec<Particle>) -> f64 {
        - self.omega / 2. * wf.laplace(&particles)
    }

    fn potential(&self, particles: &Vec<Particle>) -> f64 {
        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum_scaled_z(&self.lambda)).sum();
        self.omega / 2. * squared_position_sum
    }
}
