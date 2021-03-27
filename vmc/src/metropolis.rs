use rand::thread_rng;
use rand::distributions::{Uniform, Distribution};
use crate::{
    System,
    Particle,
    montecarlo::SampledValues,
};


pub enum MetropolisResult {
    Accepted(SampledValues),
    Rejected,
}

/// Trait for Metropolis samplers.
pub trait Metropolis {
    fn new(step_size: f64) -> Self;
    fn step(&mut self, sys: &mut System, non_interacting: bool) -> MetropolisResult;
    fn hastings_check(acceptance_factor: f64) -> bool {
        if acceptance_factor >= 1. { true }
        else {
            let mut rng = thread_rng();
            let uniform = Uniform::new(0., 1.);
            uniform.sample(&mut rng) < acceptance_factor
        }
    }
    fn sample(sys: &mut System, non_interacting: bool) -> MetropolisResult {
        let d_wf_deriv = sys.wavefunction.gradient_alpha(&sys.particles); 
        let d_energy = if non_interacting { sys.hamiltonian.energy_non_interacting(&sys.wavefunction, &mut sys.particles) }
                                     else { sys.hamiltonian.energy(&sys.wavefunction, &mut sys.particles) };

        MetropolisResult::Accepted(SampledValues {
            energy: d_energy,
            energy_squared: d_energy.powi(2),
            wf_deriv: d_wf_deriv,
            wf_deriv_times_energy: d_wf_deriv * d_energy,
        })
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

    fn step(&mut self, sys: &mut System, non_interacting: bool) -> MetropolisResult {
        let wf_old: f64 = if non_interacting { sys.wavefunction.evaluate_non_interacting(&sys.particles) }
                                        else { sys.wavefunction.evaluate(&sys.particles) };

        let next_step = sys.random_particle_change(self.step_size);

        let wf_new: f64 = if non_interacting { sys.wavefunction.evaluate_non_interacting(&next_step) }
                                        else { sys.wavefunction.evaluate(&next_step) };

        if Self::hastings_check(wf_new.powi(2) / wf_old.powi(2)) {
            sys.particles = next_step;
            Self::sample(sys, non_interacting)
        } else {
            MetropolisResult::Rejected
        }
    }
}


/// Struct for representing an importance sampling Metropolis algorithm.
/// Implements the Metropolis trait.
pub struct ImportanceMetropolis;

impl Metropolis for ImportanceMetropolis {
    /// Makes a new `ImportanceMetropolis` struct.
    fn new(_: f64)  -> Self { Self }

    fn step(&mut self, sys: &mut System, non_interacting: bool) -> MetropolisResult {
        let (next_step, i) = sys.quantum_force_particle_change();

        /* // Old Greens below
        fn greens(x: &Particle, y: &Particle) -> f64 {
            let mut result: f64 = 0.;
            for j in 0..x.dim { // This is a vector sum + scalar product
                // 0.0025 here is the same as 0.5 * 0.005
                result += (x.position[j] - y.position[j] - 0.0025 * y.qforce[j]).powi(2);
            }
            result = (- result / 0.01).exp(); // Ignoring denominator of Greens since it cancels later
            result
        }

        let first_factor = greens(&sys.particles[i], &next_step[i]) / greens(&next_step[i], &sys.particles[i]);
        let second_factor =  sys.wavefunction.evaluate(&next_step).powi(2) / sys.wavefunction.evaluate(&sys.particles).powi(2);
        let acceptance_factor = first_factor * second_factor; */

        let mut green = 0.;
        for j in 0..sys.dimensionality {
            let first = sys.particles[i].position[j] - next_step[i].position[j] - 0.0025 * next_step[i].qforce[j];
            let second = next_step[i].position[j] - sys.particles[i].position[j] - 0.0025 * sys.particles[i].qforce[j];
            green += first.powi(2) - second.powi(2);
        }

        let wf_old: f64 = if non_interacting { sys.wavefunction.evaluate_non_interacting(&sys.particles) }
                                        else { sys.wavefunction.evaluate(&sys.particles) };

        let wf_new: f64 = if non_interacting { sys.wavefunction.evaluate_non_interacting(&next_step) }
                                        else { sys.wavefunction.evaluate(&next_step) };

        let acceptance_factor = (green / 0.01).exp() * wf_new.powi(2) / wf_old.powi(2);

        if Self::hastings_check(acceptance_factor) {
            sys.particles = next_step;
            Self::sample(sys, non_interacting)
        } else {
            MetropolisResult::Rejected
        }
    }
}
