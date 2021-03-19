use crate::particle::Particle;


//local energy for simple gaussian wavefunction
pub fn local_energy_analytical(alpha: &f64, dim: usize,  particles: &Vec<Particle>)  -> f64{
    let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
    let energy =  (*alpha as f64) * (particles.len() as f64) * (dim as f64) + (0.5  - 2. * (*alpha as f64).powf(2.)) * (squared_position_sum as f64);
    return energy
}
//driftforce
pub fn _drift_force_analytical (alpha:usize, particle: Particle) -> Vec<f64> {
    let particle_position  = particle.position;
    let drift_force = particle_position.iter().map(|x| -4. * alpha as f64 * *x as f64).collect();
    return drift_force
}
/* //squared local energy
fn local_energy_squared_analytical() -> f64{
    let energy2 = local_energy_analytical(alpha, n_particles, dim, particles: &Vec<Particle>).powf(2.)
}
//analytic simple gaussian wavefunction
fn analytic_wavefunction_simple_guassian() {
    let squared_position_sum: f64 = particles.iter().map(|x| -alpha * x.squared_sum()).sum();
    let psi  = squared_position_sum.exp()
} */
