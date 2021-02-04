// Import structs and make them publicly accessible
mod particle;
mod solver;
mod system;
pub use particle::Particle;
pub use system::System;
pub use system::HarmonicTrap;
pub use solver::BruteForceMetropolis;


fn main() {
    let boson = Particle::new(&[3., 2., 1.]);
    println!("{:?}", boson);
}
