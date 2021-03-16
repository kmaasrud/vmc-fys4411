mod particle;
mod metropolis;
mod system;
mod wavefunction;
mod hamiltonian;
mod montecarlo;
mod writer;
pub use particle::Particle;
pub use system::System;
pub use metropolis::{Metropolis, MetropolisResult, BruteForceMetropolis};
pub use wavefunction::{WaveFunction, GaussianWaveFunction};
pub use hamiltonian::{Hamiltonian, HarmonicOscillator};
use montecarlo::monte_carlo;

use std::time:: Instant;



fn main() {
    let alpha = 0.5;
    let mc_cycles = 1_000_000;
    let n_particles = 100 ;
    let dimensions = 1;
    let step_size = 0.1;
    
    let start = Instant::now();
    let wf: GaussianWaveFunction = GaussianWaveFunction::new(alpha);
    let ham: HarmonicOscillator = HarmonicOscillator::elliptical(1.0, 1.0);
    let mut test_system: System<GaussianWaveFunction, HarmonicOscillator> = System::distributed(n_particles, dimensions, wf, ham, 0.1);
    let mut metro: BruteForceMetropolis = BruteForceMetropolis::new(step_size);
    println!("Energy from monte carlo calculations {}", monte_carlo(mc_cycles, &mut test_system, &mut metro)); 
    let duration = start.elapsed();
    println!("Time used in seconds {:?} = {:?} min",duration, duration/60);

}
