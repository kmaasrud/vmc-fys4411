use crate::Particle;



//local energy for simple gaussian wavefunction
fn local_energy_analytical(particle: Particle, particles: &Vec<Particle>) -> f64 {
    //let alpha = vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    //let dim = 0..=3; 
    let alpha:f64   = 0.5;
    let dim:f64     = 1.;
    let n          = 1.;
    let omega: f64  = 1.;
    
    let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();

    alpha* n * dim + (0.5 * omega.powf(2.) - 2. * alpha.powf(2.))* squared_position_sum

}
//driftforce
fn drift_force_analytical (particle: Particle) -> Vec<f64>{
    let particle_position  = particle.position;
    let alpha:f64   = 0.5;
    let dim:f64     = 1.;
    let omega: f64 = 1.;
    let n = 1.;
    particle_position.iter().map(|x| -4. * alpha * x).collect()
    //-4.*alpha*particle_position.iter()
}