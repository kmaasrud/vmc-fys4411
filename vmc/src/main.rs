mod particle;
mod metropolis;
mod system;
mod wavefunction;
mod hamiltonian;
mod montecarlo;
mod threadpool;
mod analytical;

pub use particle::Particle;
pub use system::System;
pub use metropolis::{Metropolis, MetropolisResult, BruteForceMetropolis};
pub use wavefunction::{WaveFunction, GaussianWaveFunction};
pub use hamiltonian::Hamiltonian;
use montecarlo::monte_carlo;
use threadpool::ThreadPool;

use std::time:: Instant;

extern crate num_cpus;

use std::fs::OpenOptions;
use std::io::{BufWriter, Write};


fn main() {
    //let alpha = 0.5;
    //let n_particles = 1 ;
    //let dimensions = 1;
    let step_size = 1.0;
    let total_mc_cycles = 10_000;
    


    let alpha_list: Vec<f64> = vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    let particle_list = vec![1,10,100];

    let variance = 1;
    let accept_ratio = 1;

    let header = "Alpha,Energy,Energy2,Variance,AcceptRatio,ElapesdTime\n";
    
    let start = Instant::now();

    let cpus = num_cpus::get();
    let mc_cycles:usize = total_mc_cycles / cpus;
    println!("{}", mc_cycles);
    let pool = ThreadPool::new(cpus as u8);
    for _ in 0..cpus {
        let alc = alpha_list.clone();
        let plc = particle_list.clone();
        pool.execute(move ||run_sim(alc.clone(), plc.clone(), variance, accept_ratio, header, start, step_size, mc_cycles));
    }

    println!("All {} cores now executing, waiting for them to finish", cpus);
    pool.join_all();

    println!("Total time spent: {:?}", start.elapsed());


}



fn run_sim(alpha_list: Vec<f64>, particle_list:Vec<usize>, variance:i32, accept_ratio:i32, header: &str, start:Instant, step_size:f64, mc_cycles:usize) {
    for dim in 1..=3{
        //println!("dim: {}",dim);

        for n_particles in particle_list.iter(){
            println!("Calculating: dim: {}, n_part {}, {:?}",dim, n_particles, std::thread::current().id());
            //dummypath
            //let path = format!("./data/dummydata/dummy_{}D_{}_particles.csv", dim, n_particles);
            //path for analytical results
            //let path = format!("./data/analytic/experiment_{}D_{}_particles_ana.csv", dim, n_particles);
            //path numerical results
            
            let path = format!("./data/numeric/{:?}/", std::thread::current().id());
            if std::path::Path::new(&path).exists() == false {
                std::fs::create_dir_all(path).expect("Can't create folder");
            }
            let filepath = format!("./data/numeric/{:?}/experiment_{}D_{}_n_part.csv", std::thread::current().id(), dim, n_particles );
            //let path = format!("./data/non_paralell/numeric/experiment_{}D_{}_particles_num_{:?}.csv", dim, n_particles, std::thread::current().id());
            //let path_ana = format!("./data/non_paralell/analytic/experiment_{}D_{}_particles_num_{:?}.csv", dim, n_particles, std::thread::current().id());
            let file = OpenOptions::new()
                        .read(true)
                        .append(true)
                        .create(true)
                        .open(filepath)
                        .expect("Unable to open file");
            //let file_ana = OpenOptions::new().read(true).append(true).create(true).open(path_ana).expect("Unable to open file");
            let mut f = BufWriter::new(file);
            //let mut f_ana = BufWriter::new(file_ana);
            
            f.write_all(header.as_bytes()).expect("Unable to write data"); 
            //f_ana.write_all(header.as_bytes()).expect("Unable to write data"); 


            for alpha in alpha_list.iter(){
                let wf: GaussianWaveFunction = GaussianWaveFunction::new(*alpha);
                let ham: Hamiltonian = Hamiltonian::elliptical(1.0, 1.0);
                let mut test_system: System<GaussianWaveFunction> = System::distributed(*n_particles, dim, wf, ham, 0.1);
                let mut metro: BruteForceMetropolis = BruteForceMetropolis::new(step_size);
                //println!("Energy from monte carlo calculations {}", monte_carlo(mc_cycles, &mut test_system, &mut metro)); 
                let energy = monte_carlo(mc_cycles, &mut test_system, &mut metro); 
                let energy2 = energy.powi(2);

                //let energy_ana = local_energy_analytical(alpha, dim, &test_system.particles);
                //let energy2_ana = energy_ana.powi(2);

                let duration = start.elapsed();
                //println!("Time used in seconds {:?} = {:?} min",duration, duration/60);
                let data = format!("{},{},{},{},{},{:?}\n", alpha, energy, energy2, variance, accept_ratio, duration);
                //let data_ana = format!("{},{},{},{},{},{:?}\n", alpha, energy_ana, energy2_ana, variance, accept_ratio, duration);
                
                f.write_all(data.as_bytes()).expect("Unable to write data");
                //f_ana.write_all(data_ana.as_bytes()).expect("Unable to write data");
            }
        }
    }
}
