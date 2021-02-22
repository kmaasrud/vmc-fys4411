use crate::System;
use crate::WaveFunction;
use crate::{Metropolis, MetropolisResult};

fn monte_carlo<T: Metropolis, V: WaveFunction>(n: usize, sys: &mut System<V>, metro: &mut T) -> f64 {
    let mut result: f64 = 0.;

    // Just some gibberish for now, but the general structure holds
    for _ in 0..n {
        match metro.step(sys) {
            MetropolisResult::Accepted(val) => result += val,
            MetropolisResult::Rejected => result += 1.,
        }
    }
    result
}
