mod particle;
mod metropolis;
mod system;
mod wavefunction;
mod hamiltonian;
mod montecarlo;
pub use particle::Particle;
pub use system::System;
pub use metropolis::{Metropolis, MetropolisResult};
pub use wavefunction::{WaveFunction, GaussianWaveFunction};
pub use hamiltonian::{Hamiltonian, HarmonicOscillator};


fn main() {
    let wf: GaussianWaveFunction = GaussianWaveFunction::new(0.5);
    let ham: HarmonicOscillator = HarmonicOscillator::elliptical(0.5, 1.);
    let test_system: System<GaussianWaveFunction, HarmonicOscillator> = System::distributed(1, 3, wf, ham, 2.);
    println!("{:#?}", test_system);
}
