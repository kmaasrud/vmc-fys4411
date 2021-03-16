use std::fs::OpenOptions;
use std::io::{BufWriter, Write};


pub fn writer(){
    let energy = 200;
    let energy2 = 500;
    let alpha_list = vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    let dim_list =1..=3 ;
    let particle_list = vec![1,10,100,500];
    let header = "Alpha,Energy,Energy2,Variance,AcceptRatio,ElapesdTime\n";


    for dim in dim_list{
        println!("dim: {}",dim);

        for particle in particle_list.iter(){
            println!("particle: {}",particle);
            //dummypath
            let path = format!("./data/dummydata/dummy_{}D_{}_particles.csv", dim, particle);
            //path for analytical results
            //let path = format!("./data/analytic/experiment_{}D_{}_particles_ana.csv", dim, particle);
            //path numerical results
            //let path = format!("./data/numeric/experiment_{}D_{}_particles_num.csv", dim, particle);
            
            let f = OpenOptions::new()
                        .read(true)
                        .append(true)
                        .create(true)
                        .open(path)
                        .expect("Unable to open file");
            let mut f = BufWriter::new(f);
            
            f.write_all(header.as_bytes()).expect("Unable to write data"); 

            for alpha in alpha_list.iter(){
                let data = format!("{},{},{}\n", alpha, energy, energy2);
                f.write_all(data.as_bytes()).expect("Unable to write data"); 
                }   
                
            }
        }

}
