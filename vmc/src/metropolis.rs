use rand::thread_rng;
use rand::distributions::{Uniform, Distribution};
use crate::{
    System,
    Particle,
    montecarlo::SampledValues
};


pub enum MetropolisResult {
    Accepted(SampledValues),
    Rejected,
}

/// Trait for Metropolis samplers.
pub trait Metropolis {
    fn new(step_size: f64) -> Self;
    fn step(&mut self, sys: &mut System) -> MetropolisResult;
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

impl Metropolis for BruteForceMetropolis {
    /// Makes a new `BruteForceMetropolis` struct based on a step size.
    fn new(step_size: f64) -> Self {
        Self { step_size: step_size, }
    }

    fn step(&mut self, sys: &mut System) -> MetropolisResult {
        let wf_old: f64 = sys.wavefunction.evaluate_non_interacting(&sys.particles);
        let next_step = sys.random_particle_change(self.step_size);
        let wf_new: f64 = sys.wavefunction.evaluate_non_interacting(&next_step);

        let acc_factor = self.acceptance_factor(wf_old.powi(2), wf_new.powi(2));

        if Self::hastings_check(acc_factor) {
            sys.particles = next_step;
            let d_energy = sys.hamiltonian.local_energy(&sys.wavefunction, &sys.particles);
            let d_wf_deriv = sys.wavefunction.gradient_alpha(&sys.particles); 
            MetropolisResult::Accepted(SampledValues {
                energy: d_energy,
                energy_squared: d_energy.powi(2),
                wf_deriv: d_wf_deriv,
                wf_deriv_times_energy: d_wf_deriv * d_energy,
            })
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

impl Metropolis for ImportanceMetropolis {
    /// Makes a new `ImportanceMetropolis` struct based on a step size.
    fn new(step_size: f64)  -> Self {
        Self { step_size: step_size, }
    }

    fn step(&mut self, sys: &mut System) -> MetropolisResult {
        let (next_step, i) = sys.quantum_force_particle_change();

        // Greens below
        fn greens(x: &Particle, y: &Particle, dt: f64) -> f64 {
            let mut result: f64 = 0.;
            for j in 0..x.dim { // This is a vector sum + scalar product
                result += (y.position[j] - x.position[j] - 0.5 * dt * x.qforce[j]).powi(2);
            }
            result = -(result / (2. * dt)).exp(); // Ignoring denominator of Greens since it cancels later
            result
        }

        let acc_num = greens(&sys.particles[i], &next_step[i], self.step_size) * sys.wavefunction.evaluate(&next_step).powi(2);
        let acc_deno = greens(&next_step[i], &sys.particles[i], self.step_size) * sys.wavefunction.evaluate(&sys.particles).powi(2);

        if Self::hastings_check(acc_num / acc_deno) {
            sys.particles = next_step;
            let d_energy = sys.hamiltonian.energy(&sys.wavefunction, &sys.particles);
            let d_wf_deriv = sys.wavefunction.gradient_alpha(&sys.particles); 
            MetropolisResult::Accepted(SampledValues {
                energy: d_energy,
                energy_squared: d_energy.powi(2),
                wf_deriv: d_wf_deriv,
                wf_deriv_times_energy: d_wf_deriv * d_energy,
            })
        } else {
            MetropolisResult::Rejected
        }
    }
}
