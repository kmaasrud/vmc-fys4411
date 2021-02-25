use rand::thread_rng;
use rand::distributions::{Uniform, Distribution};
use crate::{System, WaveFunction};

/// Trait for Metropolis samplers.
pub trait Metropolis {
    fn step<T: WaveFunction>(&mut self, sys: &mut System<T>) -> MetropolisResult;
    // This'll probably need more generalization, but works for now
    fn acceptance_factor(&mut self, old_val: f64, new_val: f64) -> f64 {
        (old_val / new_val).min(1.)
    }
}

pub enum MetropolisResult {
    Accepted(f64),
    Rejected,
}

/// Struct for representing a brute force Metropolis algorithm.
/// Implements the Metropolis trait.
pub struct BruteForceMetropolis {
    step_size: f64,
}

impl BruteForceMetropolis {
    /// Makes a new `BruteForceMetropolis` struct based on a step size.
    fn new(step_size: f64) -> Self {
        Self{ step_size: step_size, }
    }
}

impl Metropolis for BruteForceMetropolis {
    fn step<T: WaveFunction>(&mut self, sys: &mut System<T>) -> MetropolisResult {
        let mut rng = thread_rng();
        let uniform = Uniform::new(0., 1.);

        let wf_old: f64 = sys.wavefunction.evaluate(&sys.particles);
        let next_step = sys.random_particle_change(self.step_size);
        let wf_new: f64 = sys.wavefunction.evaluate(&next_step);

        if uniform.sample(&mut rng) < self.acceptance_factor(wf_old.powi(2), wf_new.powi(2)) {
            sys.particles = next_step.clone();
            MetropolisResult::Accepted(wf_new)
        } else {
            MetropolisResult::Rejected
        }
    }
}
