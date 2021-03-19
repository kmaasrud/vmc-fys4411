use crate::Particle;

let hbar = 1.0;
let m = 1.0;
let alpha = vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
let dim = 0..=3; 
let omega = 1.0;


//local energy for simple gaussian wavefunction
fn local_energy_analytical(dim: usize, N: usize, alpha: vec, ) --> f64 {
    alpha* N * dim + (0.5 * omega.powi(2) - 2 * alpha.powi(2))* position.powi(2).sum()

}