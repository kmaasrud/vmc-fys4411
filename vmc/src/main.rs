// Import structs and make them publicly accessible
mod particle;
mod system;
pub use particle::Particle;
pub use system::System;

fn main() {
    let boson = Particle::new(&[3., 2., 1.]);
    println!("{:?}", boson);
}