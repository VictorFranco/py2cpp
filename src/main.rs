use std::fs::File;          // file libraries
use std::io::prelude::*;

fn main() {
    let mut file = File::open("./src/ProyectoFinal.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}
