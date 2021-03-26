use crate::{System, Metropolis, MetropolisResult};

/// Collection of values that are integrated over
#[derive(Clone, Debug)]
pub struct SampledValues {
    pub energy: f64,
    pub energy_squared: f64,
    pub wf_deriv: f64,
    pub wf_deriv_times_energy: f64,
}

impl SampledValues {
    fn new() -> Self { SampledValues { energy: 0., energy_squared: 0., wf_deriv: 0., wf_deriv_times_energy: 0. } }
    fn add_to_sum(&mut self, dvals: &SampledValues) {
        self.energy += dvals.energy;
        self.energy_squared += dvals.energy_squared;
        self.wf_deriv += dvals.wf_deriv;
        self.wf_deriv_times_energy += dvals.wf_deriv_times_energy;
    }

    fn scale_by(&mut self, factor: f64) {
        self.energy /= factor;
        self.energy_squared /= factor;
        self.wf_deriv /= factor;
        self.wf_deriv_times_energy /= factor;
    }
}

/// Does Monte Carlo integration over the WaveFunction of a System, using a given Metropolis
/// algorithm.
///
/// **Parameters**:
/// - n: usize -- The number of Monte Carlo cycles to perform
/// - sys: &mut System<V: WaveFunction, W: Hamiltonian> -- Reference to a System struct containing a WaveFunction and a Hamiltonian
/// - metro: &mut T where T: Metropolis -- Reference to a Metropolis struct
pub fn monte_carlo<T: Metropolis>(n: usize, sys: &mut System, metro: &mut T) -> SampledValues {
    let pre_steps = n / 4;
    let mut result = SampledValues::new();

    // Run a couple of steps to get the system into equilibrium
    for _ in 0..pre_steps {
        match metro.step(sys) {
            MetropolisResult::Accepted(vals) => result = vals,
            MetropolisResult::Rejected => {},
        }
    }

    let mut prev_dvals = result.clone();

    for _ in 0..n {
        match metro.step(sys) {
            MetropolisResult::Accepted(dvals) => {
                // println!("{}", dvals.energy);
                result.add_to_sum(&dvals);
                prev_dvals = dvals;
            },
            MetropolisResult::Rejected => {
                result.add_to_sum(&prev_dvals);
                // println!("{}", prev_dvals.energy);
            },
        }
    }
    result.scale_by(n as f64);
    result
}
