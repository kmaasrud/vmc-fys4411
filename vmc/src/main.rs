// Import structs and make them publicly accessible
mod particle;
mod solver;
mod system;
pub use particle::Particle;
pub use system::System;
pub use system::HarmonicTrap;
pub use solver::SimpleMetropolis;


fn main() {
    let boson = Particle::new(&[3., 2., 1.]);
    println!("{:?}", boson);
    SimpleMetropolis::solve(&mut SimpleMetropolis{ curr_step: 2., next_step: 0., acceptance_factor: 0., rand: 0., time: 0 }, -10., 10)
}
