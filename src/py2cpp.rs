use regex::Regex;
use std::collections::HashMap;
use crate::instructions::{print, declare};

// head of declared function
const HEAD_DEC_FUN: &str = r"(?m)def\s([a-zA-Z][a-zA-Z_-]*)\(((([a-zA-Z][a-zA-Z0-9]*),?)*)\):";

// declared function with body
const DEC_FUN: &str = r"(?m)def\s[a-zA-Z][a-zA-Z_-]*\(.*\):\n((\s{4,}.*\n)*)";

const PARAMS: &str = r"[a-zA-Z][a-zA-Z0-9]*";

const INSTRUCTIONS: &str = r"(?m)(.*)\n";

const SHIFT_LEFT: &str = r"(?m)\s{4,}(.*)\n";

const RETURN: &str = r"return (.*)";

const MAIN: &str = r"(?m)^\S{4,}.*$";

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    String,
    Void,
    Undefined
}

#[derive(Debug)]
struct Param {
    type_: Type,
    name: String
}

#[derive(Debug)]
pub struct Argument {
    pub type_: Type,
    pub content: String
}

#[allow(unused)]
#[derive(Debug)]
pub enum Instruction {
    CreateVar { type_: Type, name: String, value: String },
    CallFun { name: String, arguments: Vec<Argument> },
    Loop { start: String, end: String, content: Vec<Instruction> },
    Return (String)
}

#[derive(Debug)]
struct Function {
    type_: Type,
    name: String,
    params: Vec<Param>,
    body: Vec<Instruction>
}

#[derive(Debug, Clone, PartialEq)]
struct Library {
    name: String
}

#[derive(Debug)]
struct Code {
    libraries: Vec<Library>,
    functions: Vec<Function>
}

impl Code {

    fn create_code() -> Code {
        Code {
            libraries: Vec::new(),
            functions: Vec::new()
        }
    }

    fn get_header_info(header: &str) -> (String, Vec<Param>) {
        let re = Regex::new(HEAD_DEC_FUN).unwrap();
        let cap = re.captures(header).unwrap();
        let name = cap.get(1).unwrap().as_str().to_string();    // get function name

        let params = cap.get(2).unwrap().as_str();
        let re = Regex::new(PARAMS).unwrap();
        let caps = re.captures_iter(&params);
        let mut params = Vec::new();

        for cap in caps {
            let type_ = Type::Undefined;
            let name = cap.get(0).unwrap().as_str().to_string();

            params.push(
                Param { type_, name }                           // get function params
            );
        }
        (name, params)
    }

    fn get_return_type(header: &str) -> Type {
        let re = Regex::new(RETURN).unwrap();

        if !re.is_match(header) {      // if the function has no return statement
            return Type::Void;          // then it is void type
        }
        else {
            return Type::Undefined
        }
    }

    fn add_lib<'a>(dic_of_libs: &mut HashMap<&'a str, Vec<Library>>, keyword: &'a str, names: &[&str]) {
        let mut libraries = Vec::new();
        for name in names.iter() {
            let name = name.to_string();
            libraries.push(
                Library { name }
            );
        }
        dic_of_libs.insert( keyword, libraries );
    }

    fn get_instructions(self: &mut Code, body: String) -> Vec<Instruction> {
        let mut dic_of_libs: HashMap<&str, Vec<Library>> = HashMap::new();
        Self::add_lib(&mut dic_of_libs, "cout" , &["iostream"]);

        let re = Regex::new(INSTRUCTIONS).unwrap();
        let caps = re.captures_iter(&body);
        let mut instructions: Vec<Instruction> = Vec::new();

        let re_return = Regex::new(RETURN).unwrap();

        for cap in caps {
            let content = cap.get(1).unwrap().as_str();
            let opt_instruc = print::py2code(content);
            match opt_instruc {
                Some(instruction) => {
                    instructions.push(instruction);
                    self.libraries.append(&mut dic_of_libs.get("cout").unwrap().clone());
                }
                None => {}
            }
            let opt_instruc = declare::py2code(content);
            match opt_instruc {
                Some(instruction) => instructions.push(instruction),
                None => {}
            }
            let cap_return = re_return.captures(content);
            match cap_return {
                Some(data) => {
                    let value = data.get(1).unwrap().as_str().to_string();
                    let instruction = Instruction::Return(value);
                    instructions.push(instruction);
                },
                None => {}
            }
        }
        instructions
    }

    fn get_main(self: &mut Code, py_code: &str) -> Function {
        let mut dic_of_libs: HashMap<&str, Vec<Library>> = HashMap::new();
        Self::add_lib(&mut dic_of_libs, "cout" , &["iostream"]);

        let re = Regex::new(MAIN).unwrap();
        let caps = re.captures_iter(py_code);
        let mut body = String::new();

        for cap in caps {
            let content = cap.get(0).unwrap().as_str();
            body = format!("{}{}\n", body, content);
        }

        let mut body: Vec<Instruction> = Self::get_instructions(self, body);

        body.push(
            Instruction::Return("0".to_string())
        );

        Function {
            type_: Type::Int,
            name: "main".to_string(),
            params: Vec::new(),
            body
        }
    }

    fn shift_code_left(body: &str) -> String {
        let re = Regex::new(SHIFT_LEFT).unwrap();
        let caps = re.captures_iter(&body);
        let mut body = String::new();

        for cap in caps {
            let content = cap.get(1).unwrap().as_str();
            body = format!("{}{}\n", body, content);
        }

        body
    }

    fn py2code(py_code: &str) -> Code {
        let re = Regex::new(DEC_FUN).unwrap();
        let caps = re.captures_iter(py_code);
        let mut code = Self::create_code();

        for cap in caps {
            let body = cap.get(1).unwrap().as_str();
            let body = Self::shift_code_left(body);
            let header = cap.get(0).unwrap().as_str();

            let type_: Type = Self::get_return_type(header);
            let body: Vec<Instruction> = Self::get_instructions(&mut code, body);
            let (name, params): (String, Vec<Param>) = Self::get_header_info(header);

            code.functions.push(
                Function { type_, name, params, body }
            );
        }

        let main: Function = Self::get_main(&mut code, py_code);
        code.functions.push(main);

        code.libraries.dedup();         // remove duplicate libraries

        code
    }

    fn fun2cpp(function: &Function) -> String {
        let dic_types = HashMap::from([
             (Type::Int, "int"),
             (Type::String, "string"),
             (Type::Void, "void"),
             (Type::Undefined, "undefined"),
        ]);

        // generate function header
        let type_ = dic_types.get(&function.type_).unwrap();
        let mut header = format!("{} {}(", type_, function.name);
        for (index, param) in function.params.iter().enumerate() {
            if index > 0 {
                header.push_str(", ");
            }
            let type_ = dic_types.get(&param.type_).unwrap();
            header = format!("{}{} {}", header, type_, param.name);
        }

        // generate function body
        let mut body = String::new();
        for instruction in &function.body {
            let result = match instruction {
                Instruction::CallFun { name, arguments } => {
                    print::code2cpp(name, arguments)
                },
                Instruction::CreateVar { type_, name, value } => {
                    declare::code2cpp(type_, name, value)
                }
                Instruction::Return(value) => format!("return {};", value),
                _ => String::new()
            };
            body = format!("{}    {}\n", body, result);
        }
        format!("{}) {{\n{}}}\n", header, body)
    }

    fn code2cpp(self: Code) -> String {
        // generate libraries
        let mut result = String::new();
        let mut cpp_std = false;
        for library in self.libraries.iter() {
            result = format!("{}#include <{}>\n", result, library.name);
            if library.name == "iostream" {
                cpp_std = true;
            }
        }
        result.push('\n');
        // add namespace
        if cpp_std {
            result = format!("{}using namespace std;\n\n", result);
        }
        // generate functions
        for function in self.functions.iter() {
            result = format!("{}{}\n", result, Self::fun2cpp(function));
        }
        result
    }

}

pub fn transpile(py_code: &str) -> String {
    let code: Code = Code::py2code(py_code);
    println!("{:?}", code);

    Code::code2cpp(code)
}
