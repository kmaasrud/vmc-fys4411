use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    //Create path to desired file
    let path = Path::new("./python/data/output.txt");
    let display = path.display();

    //Open path in read only mode, returns 'io::Result<File>
    let mut file = match File::open(&path){
        Err(why) => panic!("Could not open {}: {}", display, why),
        Ok(file) => file,
    };
    //Read file contents into string, returns 'io::Result<usize>'
    
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could not read {}: {}", display, why),
        Ok(_) => print!("{} contains: \n{}",display, s), 
    }
}