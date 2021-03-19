use crate::particle::Particle;


//local energy for simple gaussian wavefunction
fn local_energy_analytical(alpha: usize, n_particles: usize, dim:usize, particles: &Vec<Particle>) -> f64 {
    let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
    alpha * n * dim + (0.5  - 2. * alpha.powf(2.))* squared_position_sum

}
//driftforce
fn drift_force_analytical (alpha:usice, particle: Particle) -> Vec<f64>{
    let particle_position  = particle.position;
    let energy = particle_position.iter().map(|x| -4. * alpha * x).collect()
}
//squared local energy
fn local_energy_squared_analytical() -> f64{
    let energy2 = local_energy_analytical(alpha, n_particles, dim, particles: &Vec<Particle>).powf(2.)
}
//analytic simple gaussian wavefunction
fn analytic_wavefunction_simple_guassian() {
    let squared_position_sum: f64 = particles.iter().map(|x| -alpha * x.squared_sum()).sum();
    let psi  = squared_position_sum.exp()
}