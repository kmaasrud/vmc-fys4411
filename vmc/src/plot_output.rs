use std::fs::File;
use std::io::prelude::*;


fn main() -> std::io::Result <(), Error>{
    let mut file = File::create("file.txt")?; //read ionly mode
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    assert_eq!(contents, "hello")
    Ok(())
}