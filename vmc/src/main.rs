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


fn main() {
    let wf: GaussianWaveFunction = GaussianWaveFunction::new(0.5);
    let test_system: System<GaussianWaveFunction> = System::distributed(5, 3, wf, 2.);
    println!("{:#?}", test_system);
}
