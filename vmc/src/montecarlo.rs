use crate::System;
use crate::WaveFunction;
use crate::{Metropolis, MetropolisResult};

/// Does Monte Carlo integration over the WaveFunction of a System, using a given Metropolis
/// algorithm.
///
/// **Parameters**:
/// - n: usize -- The number of Monte Carlo cycles to perform
/// - sys: &mut System<V: WaveFunction> -- Reference to a System struct containing a WaveFunction
/// - metro: &mut T where T: Metropolis -- Reference to a Metropolis struct
fn monte_carlo<T: Metropolis, V: WaveFunction>(n: usize, sys: &mut System<V>, metro: &mut T) -> f64 {
    let mut result: f64 = 0.;
    let mut prev: f64 = sys.wavefunction.evaluate(&sys.particles);
    // Not sure how we should define the interval, and thus not sure how to calculate v, but this
    // should be pretty straight forward.
    let v: f64 = 1.;

    for _ in 0..n {
        match metro.step(sys) {
            MetropolisResult::Accepted(val) => {
                result += val;
                prev = val;
            },
            MetropolisResult::Rejected => result += prev,
        }
    }

    v * result / (n as f64)
}
