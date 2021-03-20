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
    pub fn new(step_size: f64) -> Self {
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
            MetropolisResult::Accepted(sys.hamiltonian.local_energy(&sys.wavefunction, &sys.particles))
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
        // TODO: Here we need lots of different shit
        let (next_step, i) = sys.quantum_force_particle_change(self.step_size);

        // Greens below
        let mut langevin_part: f64 = 0.;
        for j in 0..sys.particles[i].dim { // This is a vector sum + scalar product
            langevin_part += (next_step[i].position[j] - sys.particles[i].position[j] - next_step[i].qforce[j] * 0.5 * self.step_size).powi(2);
        }
        langevin_part /= -2. * self.step_size;

        let acc_factor: f64 = 1. / ((2. * 3.14 * self.step_size).powf(1.5 * (sys.particles.len() as f64) )) * langevin_part.exp();

        if Self::hastings_check(acc_factor) {
            sys.particles = next_step.clone();
            MetropolisResult::Accepted(sys.hamiltonian.local_energy(&sys.wavefunction, &sys.particles))
        } else {
            MetropolisResult::Rejected
        }
    }
}
