use crate::{System, Metropolis, MetropolisResult};

/// Collection of values that are integrated over
pub struct SampledValues {
    pub energy: f64,
    pub energy_squared: f64,
    pub wf_deriv: f64,
    pub wf_deriv_times_energy: f64,
}

impl SampledValues {
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
    let energy: f64 = sys.wavefunction.evaluate(&sys.particles);
    let wf_deriv = sys.wavefunction.gradient_alpha(&sys.particles); 
    let mut result = SampledValues {
        energy: energy,
        energy_squared: energy.powi(2),
        wf_deriv: wf_deriv,
        wf_deriv_times_energy: wf_deriv * energy,
    };
    let mut prev_dvals: SampledValues = SampledValues {
        energy: energy,
        energy_squared: energy.powi(2),
        wf_deriv: wf_deriv,
        wf_deriv_times_energy: wf_deriv * energy,
    };

    for _ in 1..n {
        match metro.step(sys) {
            MetropolisResult::Accepted(dvals) => {
                result.add_to_sum(&dvals);
                prev_dvals = dvals;
            },
            MetropolisResult::Rejected => {
                result.add_to_sum(&prev_dvals);
            },
        }
    }
    result.scale_by(n as f64);
    result
}
