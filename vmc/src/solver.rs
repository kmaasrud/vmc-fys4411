use rand::thread_rng;
use rand::distributions::{Uniform, Distribution};
use crate::{System, WaveFunction, Particle};

/// Trait for Metropolis samplers.
trait Metropolis {
    /// Evaluates whether or not a step should be takeng, based on the
    /// current curr_step and next_step. Returns a Boolean.
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
        // Initialize with random `curr_step`. Just setting empty for now, so the vector needs to be filled.
        Self{ next_step: vec![], step_size: step_size, }
    }
}

impl Metropolis for BruteForceMetropolis {
    /// Makes a new step based on `curr_step`. Also updates the value of `self.curr_step`.
    fn step<T: WaveFunction>(&mut self, sys: &mut System<T>) {
        if !self.do_change(sys) {
            sys.particles = self.next_step.clone();
        }
    }

    /// Calculates the acceptance factor based on the current step (stored in the struct) and the next step.
    fn acceptance_factor<T: WaveFunction>(&mut self, sys: &System<T>) -> f64 {
        let wf_old: f64 = sys.wavefunction.evaluate(&sys.particles);
        self.next_step = sys.random_particle_change(self.step_size);
        let wf_new: f64 = sys.wavefunction.evaluate(&self.next_step);

        let hastings_ratio: f64 = wf_new.powi(2) / wf_old.powi(2);

        // Return hastings ratio if it is smaller than 1, else 1
        hastings_ratio.min(1.)
    }
}
