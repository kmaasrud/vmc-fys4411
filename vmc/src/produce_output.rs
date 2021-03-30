use crate::{
    monte_carlo,
    ThreadPool,
    System,
    BruteForceMetropolis,
    ImportanceMetropolis,
    WaveFunction,
    Hamiltonian,
    Metropolis,
    MetropolisResult,
    montecarlo::SampledValues,
};

use std::{
    env,
    time::Instant,
    fs::{File, create_dir_all},
    path::{Path, PathBuf},
    io::prelude::*,
    
};
use num_cpus;


#[allow(dead_code)]
pub fn track_each_cycle() {
    const CSV_HEADER: &str = "MCCycles,Energy\n";
    const NON_INTERACTING: bool = false;
    const STEP_SIZE: f64 = 0.5;
    const ALPHA: f64 = 0.5;
    const DIM: usize = 3;
    const N: usize = 10;

    fn run_with<T: Metropolis>() {
        let mut path = find_cargo_root().unwrap();
        path.push("data"); path.push("track_each_cycle");
        create_dir(&path);
        path.push(format!("{}.csv", std::any::type_name::<T>().split("::").last().unwrap()));
        let mut f = create_file(&path); 
        f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data"); 

        let mut metro: T = T::new(STEP_SIZE);
        let ham: Hamiltonian = Hamiltonian::spherical();
        let wf = WaveFunction{ alpha: ALPHA, beta: 1. }; // Beta = 1, because spherical trap
        let mut system: System = System::distributed(N, DIM, wf, ham, 1.);

        let pre_steps = 1000;
        let mut result = SampledValues::new();

        // Run a couple of steps to get the system into equilibrium
        for _ in 0..pre_steps {
            match metro.step(&mut system, NON_INTERACTING) {
                MetropolisResult::Accepted(vals) => result = vals,
                MetropolisResult::Rejected => {},
            }
        }

        // Store the previous values to add if Metropolis step is rejected
        let mut prev_dvals = result.clone();

        for i in 0..10000 {
            match metro.step(&mut system, NON_INTERACTING) {
                MetropolisResult::Accepted(dvals) => {
                    result.accepted_steps += 1;
                    result.add_to_sum(&dvals);
                    prev_dvals = dvals;
                },
                MetropolisResult::Rejected => {
                    result.add_to_sum(&prev_dvals);
                },
            }
            let scaled_energy = result.energy / (i as f64);
            f.write_all(format!("{},{}\n", i, scaled_energy).as_bytes()).expect("Unable to write data");
            println!("Monte Carlo cycles: {}      Energy: {}", i, scaled_energy);
        }
    }

    let pool = ThreadPool::new(2);
    pool.execute(move || run_with::<BruteForceMetropolis>());
    pool.execute(move || run_with::<ImportanceMetropolis>());
    pool.join_all();
}


/// Produces results for dimensions 1-3, different alphas and different number of particles.
/// Does this for each core, which means we'll get to evaluate the mean over them.
/// BEWARE: This function takes a lot of time (6681.230737684s when I ran it last)
#[allow(dead_code)]
pub fn dim_and_n() {
    const CSV_HEADER: &str = "N,Dim,Alpha,Energy\n";
    const STEP_SIZE: f64 = 0.5;
    const MC_CYCLES: usize = 10_000;
    const NON_INTERACTING: bool = true;

    fn run_sim(mc_cycles: usize) {
        let mut path = find_cargo_root().unwrap();
        path.push("data"); path.push("dim_and_n");
        create_dir(&path);
        path.push(format!("{:?}.csv", std::thread::current().id()));
        let mut f = create_file(&path); 
        f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data"); 

        let alphas = [0.2, 0.25, 0.3, 0.35, 0.4, 0.45, 0.5, 0.55, 0.6, 0.65, 0.7, 0.75, 0.8];
         
        let mut metro = BruteForceMetropolis::new(STEP_SIZE);

        for dim in 1..=3 {
            println!("\tDimension {}", dim);
            for n in [1, 10, 100].iter() {
                println!("\t\tNumber of particles {}", n);
                for alpha in alphas.iter() {
                    let ham: Hamiltonian = Hamiltonian::spherical();
                    let wf = WaveFunction{ alpha: *alpha, beta: 1. }; // Beta = 1 because spherical trap
                    let mut system: System = System::distributed(*n, dim, wf, ham, 1.);
                    let vals = monte_carlo(mc_cycles, &mut system, &mut metro, NON_INTERACTING); 
                    f.write_all(format!("{},{},{},{:?}\n", n, dim, alpha, vals.energy).as_bytes()).expect("Unable to write data");
                    println!("\t\t\tAlpha value: {}, Energy: {}", alpha, vals.energy);
                }
            }
        }
    }

    let n_cpus = num_cpus::get();
    let pool = ThreadPool::new(n_cpus as u8);

    println!("Found {} cores!", n_cpus);
    println!("Running {} Monte Carlo cycles on each core.", MC_CYCLES);

    let start = Instant::now();

    for _ in 0..n_cpus {
        pool.execute(move || run_sim(MC_CYCLES));
    }
    pool.join_all();

    println!("Total time spent: {:?}", start.elapsed());
}


/// Runs the VMC for dimension 1-3, different values of alpha and different step sizes. 
/// Does this using both brute force Metropolis sampling and importance Metropolis sampling.
#[allow(dead_code)]
pub fn bruteforce_vs_importance() {
    const N: usize = 10;
    const MC_CYCLES: usize = 10000;
    const NON_INTERACTING: bool = false;
    const CSV_HEADER: &str = "StepSize,Alpha,Energy\n";
    const DIM: usize = 3;

    fn run_sim<T: Metropolis>(alpha: f64) {
        let mut path = find_cargo_root().unwrap();
        path.push("data"); path.push("dim_and_n");
        create_dir(&path);
        path.push(format!("{:?}.csv", std::thread::current().id()));
        let mut f = create_file(&path); 
        f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data"); 

        let mut metro: T = T::new(0.5);
        let ham: Hamiltonian = Hamiltonian::elliptical(2.82843); // Input value is gamma
        let wf = WaveFunction{ alpha: alpha, beta: 2.82843 }; // Set beta = gamma
        let mut system: System = System::distributed(N, DIM, wf, ham.clone(), 1.);
        let vals = monte_carlo(MC_CYCLES, &mut system, &mut metro, NON_INTERACTING); 

        let data = format!("{},{}\n", alpha, vals.energy);
        f.write_all(data.as_bytes()).expect("Unable to write data");
    }

    fn run_for_sampler<T: Metropolis>() {
        println!("Running simulations using {} algorithm...", std::any::type_name::<T>().split("::").last().unwrap());

        // Multithreading
        let n_cpus = num_cpus::get();

        println!("Spawning threadpool of {} threads, with {} Monte Carlo cycles on each", &n_cpus, &MC_CYCLES);
        let pool = ThreadPool::new(n_cpus as u8);
        let start = Instant::now();

        for cpu_i in 1..=n_cpus {
            pool.execute(move || run_sim::<T>((cpu_i as f64) / (n_cpus as f64))); //Running the simulation on each thread individually
        }
        println!("All {} threads now executing, waiting for them to finish...", n_cpus);
        pool.join_all();
        
        println!("Time spent: {:?}", start.elapsed());
    }

    // run_for_sampler::<BruteForceMetropolis>();
    run_sim::<ImportanceMetropolis>(1.); // Step size not relevant here, so 1. does nothing
}

/// Runs the VMC for dimension X, utilizing simple gradient descent in order to choose fitting alpha parameter.
/// Only done using the noninteracting case, with importance sampling
#[allow(dead_code)]
pub fn sgd_noninteracting() {
    //DINGDINGDING, DO THE WORK!
    const N: usize = 10;
    const MC_CYCLES: usize = 20000;
    const NON_INTERACTING: bool = true;
    const CSV_HEADER: &str = "StepSize,Alpha,Energy,Energy2\n";
    const dim: usize = 3;
    const step_size: f64 = 1.;
    const tolerance: f64 = 0.00001;

    fn run(start_alpha:f64, learning_rate: f64) {
        let mut alphas:Vec<f64> = vec![];
        alphas.push(start_alpha);

        let mut done: bool = false;
        let mut energies:Vec<f64> = vec![];

        let mut path = find_cargo_root().unwrap();
        //path.push("data"); path.push("sgd_noninteracting"); path.push("start-alpha");
        path.push("data"); path.push("sgd_noninteracting"); path.push("learning-rate");
        create_dir(&path);
        //path.push(format!("start-alpha_{}.csv", start_alpha));
        path.push(format!("learning-rate_{}.csv", learning_rate));
        let mut f = create_file(&path);
        f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data");

        let mut i:usize = 0;
        while !done {

            let ham: Hamiltonian = Hamiltonian::elliptical(2.82843); // Input value is gamma
            let wf = WaveFunction{ alpha: alphas[i], beta: 2.82843 }; // Set beta = gamma
            let mut system: System = System::distributed(N, dim, wf, ham, 1.);
            let mut metro: BruteForceMetropolis = BruteForceMetropolis::new(step_size);
            let vals = monte_carlo(MC_CYCLES, &mut system, &mut metro, NON_INTERACTING); 

            energies.push(vals.energy);

            let data = format!("{},{},{},{}\n", step_size, alphas[i], vals.energy, vals.energy_squared);
            f.write_all(data.as_bytes()).expect("Unable to write data");
            println!("Dimension: {} --- Alpha: {:.16} --- Step size: {:.2} --- Energy: {:.16} --- Iteration: {}", dim, alphas[i], step_size, vals.energy, i);


            let energy_deriv = 2.* (vals.wf_deriv_times_energy-vals.wf_deriv*vals.energy);
            let new_alpha: f64 = alphas[i] - learning_rate * energy_deriv;
            alphas.push(new_alpha);

            if energy_deriv.abs() < tolerance {
                println!("Tolerance is met, exiting.");
                done = true;
            } else if i > 150 {
                println!("Max lim met, exiting.");
                done = true;
            }
            //if (energies[i]-energies[i-1]).abs() < tolerance {
            //    done = true;
            //}
            i += 1;
        }
        
    }
    // Multithreading
    println!("Running simulations using BruteForceMetropolis algorithm...");
    let start_alphas:Vec<f64> = vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    let learning_rates:Vec<f64> = vec![0.00005, 0.0001, 0.0002, 0.0004, 0.0008, 0.0016, 0.0032, 0.0064];
    let start_alpha: f64 = 0.2;
    let learning_rate: f64 = 0.0004;

    println!("Spawning threadpool of 8 threads, with {} Monte Carlo cycles on each", &MC_CYCLES);
    let pool = ThreadPool::new(8);
    let start = Instant::now();

    
    //for start_alpha in start_alphas {
    for learning_rate in learning_rates {
        pool.execute(move || run(start_alpha, learning_rate)); //Running the simulation on each thread individually
    }
    println!("All threads now executing, waiting for them to finish...");
    pool.join_all();
    
    println!("Time spent: {:?}", start.elapsed());
    }

fn create_file(filepath: &PathBuf) -> File {
    match File::create(filepath) {
        Ok(f) => f,
        Err(why) => panic!("Unable to create {:?}: {}", filepath, why),
    }
}

fn create_dir(path: &PathBuf) {
    if Path::new(path).exists() == false {
        create_dir_all(path).expect("Unable to create folder");
    }
}

fn find_cargo_root() -> Option<PathBuf> {
    let mut path: PathBuf = env::current_dir().unwrap().into();
    let file = Path::new("Cargo.toml");

    loop {
        path.push(file);

        if path.is_file() {
            path.pop();
            break Some(path);
        }

        if !(path.pop() && path.pop()) {
            break None;
        }
    }
}
