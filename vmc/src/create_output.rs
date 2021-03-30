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
    env,
    time::Instant,
    fs::{File, create_dir_all},
    path::{Path, PathBuf},
    io::prelude::*,
    
};


#[allow(dead_code)]
pub fn metropolis(){
    const CSV_HEADER: &str = "Alpha,Energy\n";
    const STEP_SIZE: f64 = 0.5;
    const NON_INTERACTING: bool = false;
    const MC_CYCLES: usize = 10_000;
    const DIM : usize = 1;
    const N: usize= 10;
    
    
    fn simulate<T: Metropolis>(){
        let alphas : Vec<f64> = (1..19).map(|x| x as f64 / 18.).collect();
        
        let mut metro: T = T::new(STEP_SIZE);

        let mut path = find_cargo_root().unwrap();
        path.push("data"); path.push("ana_vs_num"); path.push(format!("{}D_{}N", (DIM as f64),N));
        create_dir(&path);
            
        path.push(format!("{}.csv", std::any::type_name::<T>().split("::").last().unwrap()));
        let mut f = create_file(&path);
        f.write_all(CSV_HEADER.as_bytes()).expect("Unable to write data");

        println!("Dimension: {}", DIM);

        
        for alpha in alphas.iter(){

            let ham: Hamiltonian = Hamiltonian::spherical();
            let wf = WaveFunction{ alpha: *alpha, beta: 1. }; // Set beta = gamma
            let mut system: System = System::distributed(N, DIM, wf, ham.clone(), 1.);
            let vals = monte_carlo(MC_CYCLES, &mut system, &mut metro, NON_INTERACTING); 
            
            let data = format!("{},{}\n", alpha, vals.energy);
            f.write_all(data.as_bytes()).expect("Unable to write data");
        }
        
    }   

    let start = Instant::now();
    let pool = ThreadPool::new(2);
    pool.execute(move || simulate::<BruteForceMetropolis>());
    pool.execute(move || simulate::<ImportanceMetropolis>());
    pool.join_all();
    println!("Total time spent: {:?}", start.elapsed());
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

fn create_dir(path: &PathBuf) {
    if Path::new(path).exists() == false {
        create_dir_all(path).expect("Unable to create folder");
    }
}

fn create_file(filepath: &PathBuf) -> File {
    match File::create(filepath) {
        Ok(f) => f,
        Err(why) => panic!("Unable to create {:?}: {}", filepath, why),
    }
}