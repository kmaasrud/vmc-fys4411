use std::fs::{File, OpenOptions};
use std::path::Path;

use std::io::{BufWriter, Write};

fn main() {
    
    let energy = 200;
    let energy2 = 500;
    let alpha_list = vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    let header = "Alpha,Energy,Energy2,Variance,AcceptRatio,ElapesdTime\n";

    let dim_list = vec![1,2,3];
    let particle_list = vec![1,10,100,500];

    for dim in &dim_list.iter() {
        for particle in &particle_list.iter(){
            for i in &alpha_list.iter(){
                let data = format!("{},{},{}\n", energy, energy2, alpha_list[i]);
                let path = format!("./python/dummydata/dummy_{}D_{}_particles.csv", dim[0], particles[0]);
                //let path = format!("./python/data/{}D_{}_particles.csv", dim[0], particles[0]);

            }
            

        }
    }

    
    let f = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(path)
                        .expect("Unable to open file");
    let mut f = BufWriter::new(f);
    f.write_all(header.as_bytes()).expect("Unable to write to file");
    f.write_all(data.as_bytes()).expect("Unable to write data");

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