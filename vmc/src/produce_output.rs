use crate::{
    monte_carlo,
    ThreadPool,
    System,
    BruteForceMetropolis,
    ImportanceMetropolis,
    WaveFunction,
    Hamiltonian,
    Metropolis,
};

use std::{
    time::Instant,
    fs::{File, create_dir_all},
    path::Path,
    io::prelude::*,
    
};
use num_cpus;



/// Produces results for dimensions 1-3, different alphas and different number of particles and
/// saves these in its own separate file. Does this a number of times corresponding to the number
/// of cores the CPU running the program has.
pub fn dim_and_n() {
    const CSV_HEADER: &str = "Alpha,Energy,Energy2,TimeElapsed\n";
    const STEP_SIZE: f64 = 1.0;
    const MC_CYCLES: usize = 1_000;
    const NON_INTERACTING: bool = true;

    fn analytical(sys: &System)  -> f64{
        let dim = sys.dimensionality;
        let n = sys.particles.len();
        let alpha = sys.wavefunction.alpha;
        let particles = &sys.particles;

        let squared_position_sum: f64 = particles.iter().map(|x| x.squared_sum()).sum();
        let energy =  (alpha as f64) * (n as f64) * (dim as f64) + (0.5  - 2. * (alpha as f64).powf(2.)) * (squared_position_sum as f64);
        return energy
    }

    fn run_sim(start: Instant, mc_cycles: usize) {
        let alphas: Vec<f64> = (0..90).map(|x| x as f64 / 100.).collect();
        let path = format!("./data/numerical/dim_and_n/{:?}/", std::thread::current().id());
        let path_ana = format!("./data/analytical/dim_and_n/{:?}/", std::thread::current().id());
        create_dir(&path);
        create_dir(&path_ana);
         
        let mut metro: BruteForceMetropolis = BruteForceMetropolis::new(STEP_SIZE);

        for dim in 1..=3 {
            for n in [1, 10, 100].iter() {
                println!("Thread {:?} is calculating -- Dimensionality: {} --  Number of particles: {}", std::thread::current().id(), dim, n);

                let mut f = create_file(&format!("{}/numerical_{}D_{}_n_part.csv", &path, dim, n));
                let mut a = create_file(&format!("{}/analytical_{}D_{}_n_part.csv", &path_ana, dim, n));
                f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data"); 
                a.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data"); 

                for alpha in alphas.iter() {
                    let ham: Hamiltonian = Hamiltonian::spherical();
                    let wf = WaveFunction{ alpha: *alpha, beta: 1. }; // Beta = 1, because spherical trap
                    let mut system: System = System::distributed(*n, dim, wf, ham, 1.);
                    let vals = monte_carlo(mc_cycles, &mut system, &mut metro, NON_INTERACTING); 
                    
                    let energy_exact = analytical(&system);
                    let energy_exact_squared = energy_exact.powi(2);
                    
                    
                    let duration = start.elapsed();
                    let data_n = format!("{},{},{},{:?}\n", alpha, vals.energy, vals.energy_squared, duration);
                    let data_a = format!("{},{},{},{:?}\n", alpha, energy_exact, energy_exact_squared, duration);
                    
                    f.write_all(data_n.as_bytes()).expect("Unable to write data");
                    a.write_all(data_a.as_bytes()).expect("Unable to write data");
                    println!("Dimension: {} --- Alpha: {:.1} --- N: {:.2} --- Energy per particle: {} --- Analytical: {:.2}", dim, alpha, n, vals.energy / (*n as f64), energy_exact);
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


/// Runs the VMC for dimension 1-3, different values of alpha and different step sizes. 
/// Does this using both brute force Metropolis sampling and importance Metropolis sampling.
pub fn bruteforce_vs_importance() {
    const N: usize = 10;
    const MC_CYCLES: usize = 10000;
    const NON_INTERACTING: bool = false;
    const CSV_HEADER: &str = "StepSize,Alpha,Energy\n";

    fn run_sim<T: Metropolis>(step_size: f64) {
        let alphas: Vec<f64> = (1..19).map(|x| x as f64 / 18.).collect();
        let mut metro: T = T::new(step_size);
        for dim in 1..=3 {
            println!("Dimension: {}", dim);
            let path = format!("./data/bruteforce_vs_importance/{}/step_size{}", std::any::type_name::<T>().split("::").last().unwrap(), step_size);
            create_dir(&path);

            let mut f = create_file(&format!("{}/{}D.csv", &path, dim));
            f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data");

            for alpha in alphas.iter() {
                let ham: Hamiltonian = Hamiltonian::elliptical(2.82843); // Input value is gamma
                // let ham: Hamiltonian = Hamiltonian::spherical();
                let wf = WaveFunction{ alpha: *alpha, beta: 2.82843 }; // Set beta = gamma
                // let wf = WaveFunction{ alpha: *alpha, beta: 1. }; // Set beta = gamma
                let mut system: System = System::distributed(N, dim, wf, ham.clone(), 1.);
                let vals = monte_carlo(MC_CYCLES, &mut system, &mut metro, NON_INTERACTING); 

                let data = format!("{},{},{}\n", step_size, alpha, vals.energy);
                f.write_all(data.as_bytes()).expect("Unable to write data");
                println!("\tAlpha: {:.2} --- Step size: {:.2} --- Energy per particle: {:.4} --- Derivative: {:.2}", alpha, step_size, vals.energy / (N as f64), vals.wf_deriv);
            }
        }
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
pub fn sgd_noninteracting() {
    //DINGDINGDING, DO THE WORK!
    const N: usize = 10;
    const MC_CYCLES: usize = 100000;
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

        let path = format!("./data/sgd_noninteracting/learning-rate");
        create_dir(&path);
        let mut f = create_file(&format!("{}/learning-rate_{}.csv", &path, learning_rate));
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
            println!("Dimension: {} --- Alpha: {} --- Step size: {:.2} --- Energy: {}", dim, alphas[i], step_size, vals.energy);


            let energy_deriv = 2.* (vals.wf_deriv_times_energy-vals.wf_deriv*vals.energy);
            let new_alpha: f64 = alphas[i] - learning_rate * energy_deriv;
            alphas.push(new_alpha);

            if energy_deriv.abs() < tolerance {
                println!("Tolerance is met, exiting.");
                done = true;
            } else if i > 100 {
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
    let learning_rates:Vec<f64> = vec![0.00005, 0.0001, 0.0002, 0.0004, 0.0008, 0.0016, 0.032, 0.064];
    let start_alpha: f64 = 0.2;

    println!("Spawning threadpool of 8 threads, with {} Monte Carlo cycles on each", &MC_CYCLES);
    let pool = ThreadPool::new(8);
    let start = Instant::now();


    for learning_rate in learning_rates {
        pool.execute(move || run(start_alpha, learning_rate)); //Running the simulation on each thread individually
    }
    println!("All threads now executing, waiting for them to finish...");
    pool.join_all();
    
    println!("Time spent: {:?}", start.elapsed());
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
