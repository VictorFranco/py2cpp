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

fn get_return_types(contents: &str) -> Vec<String> {
    let re = Regex::new(DEC_FUN).unwrap();
    let caps = re.captures_iter(contents);
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

fn py_2_vector(contents: &str) -> Vec<Function> {
    let re = Regex::new(HEAD_DEC_FUN).unwrap();
    let caps = re.captures_iter(contents);
    let mut functions = Vec::new();

    let return_types: Vec<String> = get_return_types(contents);

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

fn main() {
    let mut file = File::open("./src/ProyectoFinal.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contents = &contents.to_string();           // store file content

    let functions: Vec<Function> = py_2_vector(contents);
    println!("{:?}", functions);
}
