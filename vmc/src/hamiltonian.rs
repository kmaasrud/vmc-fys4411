use crate::System;
use crate::WaveFunction;

trait Hamiltonian {
    fn kinetic<T: WaveFunction>(&self, sys: &System<T>) -> f64;
    fn potential<T: WaveFunction>(&self, sys: &System<T>) -> f64;
    fn local_energy<T: WaveFunction>(&self, sys: &System<T>) -> f64 {
        self.kinetic(sys) + self.potential(sys)
    }
}


pub struct HarmonicOscillator {
    lambda: f64,
    omega: f64,
}

impl HarmonicOscillator {
    fn spherical(omega: f64) -> Self {
        HarmonicOscillator { lambda: 1., omega: omega }
    }
    fn elliptic(lambda: f64, omega: f64) -> Self {
        HarmonicOscillator { lambda: lambda, omega: omega }
    }
}

impl Hamiltonian for HarmonicOscillator {
    fn kinetic<T: WaveFunction>(&self, sys: &System<T>) -> f64 {
        - self.omega / 2. * sys.wavefunction.laplace(&sys.particles)
    }

    fn potential<T: WaveFunction>(&self, sys: &System<T>) -> f64 {
        let squared_position_sum: f64 = sys.particles.iter().map(|x| x.squared_sum_scaled_z(&self.lambda)).sum();
        self.omega / 2. * squared_position_sum
    }
}
