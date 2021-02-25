use rand::thread_rng;
use rand::distributions::{Uniform, Distribution};
use crate::{System, WaveFunction};

pub enum MetropolisResult {
    Accepted(f64),
    Rejected,
}

/// Trait for Metropolis samplers.
pub trait Metropolis {
    fn step<T: WaveFunction>(&mut self, sys: &mut System<T>) -> MetropolisResult;
    // This'll probably need more generalization, but works for now
    fn acceptance_factor(&mut self, old_val: f64, new_val: f64) -> f64 {
        (old_val / new_val).min(1.)
    }
    fn hastings_check(acceptance_factor: f64) -> bool {
        let mut rng = thread_rng();
        let uniform = Uniform::new(0., 1.);
        uniform.sample(&mut rng) < acceptance_factor
    }
}


/// Struct for representing a brute force Metropolis algorithm.
/// Implements the Metropolis trait.
pub struct BruteForceMetropolis {
    step_size: f64,
}

impl BruteForceMetropolis {
    /// Makes a new `BruteForceMetropolis` struct based on a step size.
    fn new(step_size: f64) -> Self {
        Self { step_size: step_size, }
    }
}

impl Metropolis for BruteForceMetropolis {
    fn step<T: WaveFunction>(&mut self, sys: &mut System<T>) -> MetropolisResult {
        let wf_old: f64 = sys.wavefunction.evaluate(&sys.particles);
        let next_step = sys.random_particle_change(self.step_size);
        let wf_new: f64 = sys.wavefunction.evaluate(&next_step);

        let acc_factor = self.acceptance_factor(wf_old.powi(2), wf_new.powi(2));

        if Self::hastings_check(acc_factor) {
            sys.particles = next_step.clone();
            MetropolisResult::Accepted(wf_new)
        } else {
            MetropolisResult::Rejected
        }
    }
}


/// Struct for representing an importance sampling Metropolis algorithm.
/// Implements the Metropolis trait.
pub struct ImportanceMetropolis {
    step_size: f64,
}

impl ImportanceMetropolis {
    /// Makes a new `ImportanceMetropolis` struct based on a step size.
    fn new(step_size: f64)  -> Self {
        Self { step_size: step_size, }
    }
}

impl Metropolis for ImportanceMetropolis {
    fn step<T: WaveFunction>(&mut self, sys: &mut System<T>) -> MetropolisResult {
        let wf_old: f64 = sys.wavefunction.evaluate(&sys.particles);
        // Here we need lots of different shit
        let next_step = sys.random_particle_change(self.step_size);
        let wf_new: f64 = sys.wavefunction.evaluate(&next_step);

        let acc_factor = 1.;

        // The below should behave the same
        if Self::hastings_check(acc_factor) {
            sys.particles = next_step.clone();
            MetropolisResult::Accepted(wf_new)
        } else {
            MetropolisResult::Rejected
        }
    }
}
