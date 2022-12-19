#[macro_use]
extern crate lazy_static;

use std::fs::File;          // file libraries
use std::io::prelude::*;
mod py2cpp;
use py2cpp::types::Code;

fn main() {
    let file_name = "./src/ProyectoFinal.txt";
    let mut file = File::open(file_name).unwrap();
    let mut py_code = String::new();
    file.read_to_string(&mut py_code).unwrap();
    let py_code = &py_code.to_string();                 // store file content

    let mut file = File::create("./src/main.cpp").unwrap();
    match Code::transpile(py_code, file_name) {
        Ok(cpp_code) => {
            // print!("{}", cpp_code);
            let mut file = File::create("./src/main.cpp").unwrap();
            file.write_all(cpp_code.as_bytes()).unwrap();       // write generated code
        },
        Err(err) => {
            file.write_all("".as_bytes()).unwrap();       // write generated code
            println!("Error: {}", err);
        }
    }
}
