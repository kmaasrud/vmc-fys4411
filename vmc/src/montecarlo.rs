use crate::particle::Particle;
use crate::wavefunction::WaveFunction;
use crate::solver::System;


pub struct MonteCarlo {
}


trait MonteCarlo {
    fn step<T>(
            &self, 
            wavefunction: &T, 
            particle: &mut Particle,
            step_size: f64) -> bool where T: WaveFunction;
}



impl MonteCarlo for BruteForceMetropolis{
    fn step<T>(
        &self, 
        wavefunction: &T, 
        particle: &mut Particle,
        step_size: f64) -> bool 
        
    where 
        T: WaveFunction, {
      let old_wavefunc = wavefunction.evaluate(&particle);
      let p_index = random::<usize>() % particle.len();
      println!(p_index);
    }
}