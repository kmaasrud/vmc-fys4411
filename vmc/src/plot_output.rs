extern crate csv;
#[macro_use]
extern crate serde_derive;
/* 
csv = "1.1"
serde_derive = "1.0" */

use std::error::Error;
use std::io;
use std::process;

#[derive(Debug, Sterialize)]

struct Record{
    dim:            usize,
    particle:       usize,
    alpha:          f32,
    energy:         f64,
    energy2:        f64,
    variance:       f64,
    accept_ratio:   f64,
    ellapsed_time:  i32,

}


fn write_to_csv() -> Result<(), Box<Error>> {
    let mut writer = csv::Reader::from_reader(io::stdin());
}

/* let particle_list: Vec<usize> =  vec![2, 10, 100, 500];
let dim_list: Vec<usize> =  vec![1, 3, 3];
let alpha = vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]
println!("{}, {}", dim_list.len(), particle_list.len());

for dim in dim_list.iter() {
    println!("{}", dim);
    
    for particle in particle_list.iter() {
        println!("{}", particle);
        let wf: GaussianWaveFunction = GaussianWaveFunction::new(alpha);
        let ham: HarmonicOscillator = HarmonicOscillator::elliptical(0.5, 1.);
        let mut test_system: System<GaussianWaveFunction, HarmonicOscillator> = System::distributed(*particle, *dim, wf, ham, 0.1);
        let mut metro: BruteForceMetropolis = BruteForceMetropolis::new(step_size);
        println!("{}", monte_carlo(mc_cycles, &mut test_system, &mut metro));
    }
} */