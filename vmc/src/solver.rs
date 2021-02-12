use rand::thread_rng;
use rand::distributions::{Uniform, Distribution};
use crate::{System, WaveFunction, Particle};

/// Trait for Metropolis samplers.
trait Metropolis {
    fn do_change<T: WaveFunction>(&mut self, sys: &System<T>) -> bool {
        let mut rng = thread_rng();
        let uniform = Uniform::new(0., 1.);

        if uniform.sample(&mut rng) < self.acceptance_factor(sys) {
            true
        } else {
            false
        }
    }

    fn step<T: WaveFunction>(&mut self, sys: &mut System<T>);
    fn acceptance_factor<T: WaveFunction>(&mut self, sys: &System<T>) -> f64;
}

/// Struct for representing a brute force Metropolis algorithm.
/// Implements the Metropolis trait.
pub struct BruteForceMetropolis {
    next_step: Vec<Particle>,
    step_size: f64,
}

impl BruteForceMetropolis {
    /// Makes a new `BruteForceMetropolis` struct based on a step size.
    fn new(step_size: f64) -> Self {
        Self{ next_step: vec![], step_size: step_size, }
    }
}

impl Metropolis for BruteForceMetropolis {
    fn step<T: WaveFunction>(&mut self, sys: &mut System<T>) {
        if !self.do_change(sys) {
            sys.particles = self.next_step.clone();
        }
    }

    fn acceptance_factor<T: WaveFunction>(&mut self, sys: &System<T>) -> f64 {
        let wf_old: f64 = sys.wavefunction.evaluate(&sys.particles);
        self.next_step = sys.random_particle_change(self.step_size);
        let wf_new: f64 = sys.wavefunction.evaluate(&self.next_step);

        let hastings_ratio: f64 = wf_new.powi(2) / wf_old.powi(2);

        // Return hastings ratio if it is smaller than 1, else 1
        hastings_ratio.min(1.)
    }
}
