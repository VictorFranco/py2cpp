#[macro_use]
extern crate lazy_static;

use std::fs::File;          // file libraries
use std::io::prelude::*;
mod types;
mod py2cpp;
mod constants;
mod instructions;
mod infer;

fn main() {
    let mut file = File::open("./src/ProyectoFinal.txt").unwrap();
    let mut py_code = String::new();
    file.read_to_string(&mut py_code).unwrap();
    let py_code = &py_code.to_string();                 // store file content

    let cpp_code: String = py2cpp::transpile(py_code);
    print!("{}", cpp_code);
    let mut file = File::create("./src/main.cpp").unwrap();
    file.write_all(cpp_code.as_bytes()).unwrap();       // write generated code
}
