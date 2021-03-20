use crate::{
    monte_carlo,
    ThreadPool,
    System,
    BruteForceMetropolis,
    ImportanceMetropolis,
    GaussianWaveFunction,
    Hamiltonian,
    Metropolis,
};

use std::{
    time::Instant,
    fs::{File, create_dir_all},
    path::Path,
    io::prelude::*
};
use num_cpus;
use itertools_num::linspace;


/// Produces results for dimentions 1-3, different alphas and different number of particles and
/// saves these in its own separate file. Does this a number of times corresponding to the number
/// of cores the CPU running the program has.
pub fn dim_and_n() {
    const CSV_HEADER: &str = "Alpha,Energy,Energy2,TimeElapsed\n";
    const STEP_SIZE: f64 = 1.0;
    const ALPHAS: [f64; 8] = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    const MC_CYCLES: usize = 10_000;

    fn run_sim(start: Instant, mc_cycles: usize) {
        let path = format!("./data/dim_and_n/{:?}/", std::thread::current().id());
        create_dir(&path);

        for dim in 1..=3 {
            for n in [1, 10, 100, 500].iter() {
                println!("Thread {:?} is calculating -- Dimensionality: {} --  Number of particles: {}", std::thread::current().id(), dim, n);

                let mut f = create_file(&format!("{}/experiment_{}D_{}_n_part.csv", &path, dim, n));
                f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data"); 

                for alpha in ALPHAS.iter() {
                    let wf: GaussianWaveFunction = GaussianWaveFunction::new(*alpha);
                    let ham: Hamiltonian = Hamiltonian::spherical();
                    let mut system: System<GaussianWaveFunction> = System::distributed(*n, dim, wf, ham, 0.1);
                    let mut metro: BruteForceMetropolis = BruteForceMetropolis::new(STEP_SIZE);
                    let energy = monte_carlo(mc_cycles, &mut system, &mut metro); 
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
        pool.execute(move || run_sim(start, mc));
    }

    println!("All cores now executing, waiting for them to finish...");
    pool.join_all();

    println!("Total time spent: {:?}", start.elapsed());
}


pub fn bruteforce_vs_importance() {
    const N: usize = 100;
    const ALPHAS: [f64; 8] = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    const MC_CYCLES: usize = 10_000;
    const CSV_HEADER: &str = "StepSize,Alpha,Energy,Energy2\n";

    fn run_for_sampler<T: Metropolis>() {
        println!("Running simulations using brute force Metropolis algorithm...");
        let start = Instant::now();
        for dim in 1..=3 {
            println!("Dimension: {}", dim);
            let path = format!("./data/bruteforce_vs_importance/{:?}/", std::any::type_name::<T>());
            create_dir(&path);

            let mut f = create_file(&format!("{}/{}D.csv", &path, dim));
            f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data"); 
            for step_size in linspace(0.1, 1., 10) {
                for alpha in ALPHAS.iter() {
                    let wf: GaussianWaveFunction = GaussianWaveFunction::new(*alpha);
                    let ham: Hamiltonian = Hamiltonian::spherical();
                    let mut system: System<GaussianWaveFunction> = System::distributed(N, dim, wf, ham, 0.1);
                    let mut metro: T = T::new(step_size);
                    let energy = monte_carlo(MC_CYCLES, &mut system, &mut metro); 
                    let energy2 = energy.powi(2);

                    let data = format!("{},{},{},{}\n", step_size, alpha, energy, energy2);
                    f.write_all(data.as_bytes()).expect("Unable to write data");
                }
            } 
        }
        println!("Time spent: {:?}", start.elapsed());
    }

    run_for_sampler::<BruteForceMetropolis>();
    run_for_sampler::<ImportanceMetropolis>();
}

fn create_file(filepath: &str) -> File {
    match File::create(&Path::new(filepath)) {
        Ok(f) => f,
        Err(why) => panic!("Unable to create {}: {}", filepath, why),
    }
}

fn create_dir(path: &str) {
    if Path::new(&path).exists() == false {
        create_dir_all(&path).expect("Unable to create folder");
    }
}
