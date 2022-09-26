use std::fs::File;          // file libraries
use std::io::prelude::*;
mod py2cpp;

fn main() {
    let mut file = File::open("./src/ProyectoFinal.txt").unwrap();
    let mut py_code = String::new();
    file.read_to_string(&mut py_code).unwrap();
    let py_code = &py_code.to_string();           // store file content

    let cpp_code: String = py2cpp::transpile(py_code);
    print!("{}", cpp_code);
}