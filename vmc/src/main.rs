mod particle;
mod metropolis;
mod system;
mod wavefunction;
mod hamiltonian;
mod montecarlo;
pub use particle::Particle;
pub use system::System;
pub use metropolis::{Metropolis, MetropolisResult};
pub use wavefunction::WaveFunction;


fn main() {
    let mut boson = Particle::new(3);
    boson.position = vec![2., 8., 1.];
    println!("The following boson: {:?}", boson);
    println!("Has the following squared sum: {} (nice)", boson.squared_sum());
}
