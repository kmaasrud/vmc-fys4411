// Import Particle struct and make it publicly accessible
mod particle;
pub use particle::Particle;

fn main() {
    let boson = Particle::new(&[3., 2., 1.]);
    println!("{:?}", boson);
}