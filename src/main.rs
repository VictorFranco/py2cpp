use std::fs::File;          // file libraries
use std::io::prelude::*;
use regex::Regex;

#[allow(dead_code)]
#[derive(Debug)]
struct Data {
    name: String,
    type_: String
}

#[allow(dead_code)]
#[derive(Debug)]
struct Function {
    name: String,
    type_: String,
    params: Vec<Data>,
}

// head of declared function
const HEAD_DEC_FUN:&str = r"def\s([a-zA-Z][a-zA-Z_-]*)\(([a-zA-Z][a-zA-Z0-9]*),?([a-zA-Z][a-zA-Z0-9]*)*\):";

// declared function with body
const DEC_FUN:&str = r"def\s[a-zA-Z][a-zA-Z_-]*\(.*\):(\n\s{4}.*)*";

// return
const RETURN:&str = r"return .*";

fn get_return_types(py_code: &str) -> Vec<String> {
    let re = Regex::new(DEC_FUN).unwrap();
    let caps = re.captures_iter(py_code);
    let mut returns = Vec::new();

    for cap in caps {
        let dec_fun = cap.get(0).unwrap().as_str();
        let re = Regex::new(RETURN).unwrap();
        if !re.is_match(dec_fun) {
            returns.push("void".to_string());
        }
        else {
            returns.push("undefined".to_string());
        }
    }
    returns
}

fn py_2_vector(py_code: &str) -> Vec<Function> {
    let re = Regex::new(HEAD_DEC_FUN).unwrap();
    let caps = re.captures_iter(py_code);
    let mut functions = Vec::new();

    let return_types: Vec<String> = get_return_types(py_code);

    for (index, cap) in caps.enumerate() {
        let mut params = Vec::new();

        for i in 2..cap.len() {
            match cap.get(i) {
                Some(data) => params.push(
                    Data {
                        name: data.as_str().to_string(),
                        type_: "undefined".to_string(),
                    }
                ),
                None => {}
            }
        }
        let fun = Function {
            name: cap[1].to_string(),
            type_: return_types[index].to_string(),
            params
        };
        functions.push(fun);
    }
    functions
}

fn function_2_cpp(dec_fun: &Function) -> String {
    let mut result = format!("{} {} (",dec_fun.type_,dec_fun.name);
    for (index, param) in dec_fun.params.iter().enumerate() {
        if index > 0 {
            result.push_str(", ");
        }
        result = format!("{}{} {}", result, param.type_, param.name);
    }
    result = format!("{}) {{\n}}\n", result);
    result
}

fn vector_2_cpp(functions: Vec<Function>) -> String {
    let mut result = String::new();
    for function in functions.iter() {
        result = format!("{}{}\n", result, function_2_cpp(function));
    }
    result
}

fn main() {
    let mut file = File::open("./src/ProyectoFinal.txt").unwrap();
    let mut py_code = String::new();
    file.read_to_string(&mut py_code).unwrap();
    let py_code = &py_code.to_string();           // store file content

    let functions: Vec<Function> = py_2_vector(py_code);
    println!("{:?}", functions);

    let cpp_code: String = vector_2_cpp(functions);
    print!("{}", cpp_code);
}
