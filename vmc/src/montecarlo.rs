use crate::{System, WaveFunction, Metropolis, MetropolisResult, Hamiltonian};

/// Does Monte Carlo integration over the WaveFunction of a System, using a given Metropolis
/// algorithm.
///
/// **Parameters**:
/// - n: usize -- The number of Monte Carlo cycles to perform
/// - sys: &mut System<V: WaveFunction, W: Hamiltonian> -- Reference to a System struct containing a WaveFunction and a Hamiltonian
/// - metro: &mut T where T: Metropolis -- Reference to a Metropolis struct
pub fn monte_carlo<T, V, W>(n: usize, sys: &mut System<V, W>, metro: &mut T) -> f64
where T: Metropolis, V: WaveFunction, W: Hamiltonian {
    let mut result: f64 = 0.;
    let mut prev_val: f64 = sys.wavefunction.evaluate(&sys.particles);

    for _ in 1..n {
        match metro.step(sys) {
            MetropolisResult::Accepted(val) => {
                result += val;
                prev_val = val;
            },
            MetropolisResult::Rejected => result += prev_val,
        }
    }

    result / (n as f64)
}
