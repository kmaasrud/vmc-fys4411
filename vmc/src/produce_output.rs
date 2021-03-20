use crate::{
    monte_carlo,
    ThreadPool,
    System,
    BruteForceMetropolis,
    GaussianWaveFunction,
    Hamiltonian
};

use std::{
    time::Instant,
    fs::{File, create_dir_all},
    path::Path,
    io::prelude::*
};
use num_cpus;


/// Produces results for dimentions 1-3, different alphas and different number of particles and
/// saves these in its own separate file. Does this a number of times corresponding to the number
/// of cores the CPU running the program has.
pub fn dim_and_n() {
    const CSV_HEADER: &str = "Alpha,Energy,Energy2,TimeElapsed\n";
    const STEP_SIZE: f64 = 1.0;
    const ALPHAS: [f64; 8] = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    const N_LIST: [usize; 4] = [1, 10, 100, 500];
    const MC_CYCLES: usize = 10_000;

    fn run_sim(start: Instant, mc_cycles: usize) {
        let path = format!("./data/numeric/{:?}/", std::thread::current().id());
        if Path::new(&path).exists() == false {
            create_dir_all(&path).expect("Unable to create folder");
        }

        for dim in 1..=3 {
            for n in N_LIST.iter() {
                println!("Calculating: dim: {}, n_part {}, {:?}", dim, n, std::thread::current().id());

                let filepath = &format!("{}/experiment_{}D_{}_n_part.csv", &path, dim, n);
                let mut f = match File::create(&Path::new(filepath)) {
                    Ok(f) => f,
                    Err(why) => panic!("Unable to create {}: {}", filepath, why),
                };

                f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data"); 

                for alpha in ALPHAS.iter() {
                    let wf: GaussianWaveFunction = GaussianWaveFunction::new(*alpha);
                    let ham: Hamiltonian = Hamiltonian::spherical();
                    let mut test_system: System<GaussianWaveFunction> = System::distributed(*n, dim, wf, ham, 0.1);
                    let mut metro: BruteForceMetropolis = BruteForceMetropolis::new(STEP_SIZE);
                    let energy = monte_carlo(mc_cycles, &mut test_system, &mut metro); 
                    let energy2 = energy.powi(2);

                    let duration = start.elapsed();
                    let data = format!("{},{},{},{:?}\n", alpha, energy, energy2, duration);
                    
                    f.write_all(data.as_bytes()).expect("Unable to write data");
                }
            }
        }
    }

    let n_cpus = num_cpus::get();
    println!("Found {} cores!", n_cpus);

    let mc: usize = MC_CYCLES / n_cpus;
    println!("Running {} Monte Carlo cycles on each core.", mc);

    let pool = ThreadPool::new(n_cpus as u8);

    let start = Instant::now();
    for _ in 0..n_cpus {
        pool.execute(move ||run_sim(start, mc));
    }

    println!("All cores now executing, waiting for them to finish...");
    pool.join_all();

    println!("Total time spent: {:?}", start.elapsed());
}
