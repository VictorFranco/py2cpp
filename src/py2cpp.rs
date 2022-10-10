use regex::Regex;
use std::collections::HashMap;
use crate::instructions::{print, input, custom_fun, declare, r#return};

// head of declared function
const HEAD_DEC_FUN: &str = r"(?m)def\s([a-zA-Z][a-zA-Z_-]*)\(((([a-zA-Z][a-zA-Z0-9]*),?)*)\):";

// declared function with body
const DEC_FUN: &str = r"(?m)def\s[a-zA-Z][a-zA-Z_-]*\(.*\):\n((\s{4,}.*\n)*)";

const PARAMS: &str = r"[a-zA-Z][a-zA-Z0-9]*";

const INSTRUCTIONS: &str = r"(?m)(.*)\n";

const SHIFT_LEFT: &str = r"(?m)\s{4}(.*)\n";

const MAIN: &str = r"(?m)^\S{4,}.*$";

pub const NATIVE_FUNS: [&str; 2] = ["print", "input"];

pub const INTEGER: &str = r"^[+-]?\s*(\d+)$";

pub const STRING: &str = r##"^"[a-zA-Z0-9: ]*"$"##;

pub const VARIABLE: &str = r"^[a-zA-Z][a-zA-Z0-9]*$";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
pub enum Value {
    ConstValue(String),
    CallFun { name: String, arguments: Vec<Argument> },
    None
}

#[allow(unused)]
#[derive(Debug)]
pub enum Instruction {
    CreateVar { type_: Type, name: String, value: Value },
    CallFun { name: String, arguments: Vec<Argument> },
    Loop { start: String, end: String, content: Vec<Instruction> },
    Return { type_: Type, value: String }
}

#[derive(Debug)]
struct Function {
    type_: Type,
    name: String,
    params: Vec<Param>,
    body: Vec<Instruction>
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Library {
    pub name: String
}

pub fn get_libraries(names: &[&str]) -> Vec<Library> {
    let mut libraries = Vec::new();
    for name in names.iter() {
        let name = name.to_string();
        libraries.push(
            Library { name }
        );
    }
    libraries
}

#[derive(Debug)]
pub struct Code {
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

    fn get_instructions(self: &mut Code, body: String) -> Vec<Instruction> {
        let re = Regex::new(INSTRUCTIONS).unwrap();
        let caps = re.captures_iter(&body);
        let mut body: Vec<Instruction> = Vec::new();

        for cap in caps {
            let content = cap.get(1).unwrap().as_str();
            let results = [
                print::py2code(content, "true"),
                input::py2code(content, "false"),
                declare::py2code(content),
                custom_fun::py2code(&mut body, content),
                r#return::py2code(&mut body, content)
            ];
            for result in results {
                match result {
                    Some((mut instructions, mut libraries)) => {
                        body.append(&mut instructions);
                        self.libraries.append(&mut libraries);
                    }
                    None => {}
                }
            }
        }
        body
    }

    fn get_main(self: &mut Code, py_code: &str) -> Function {
        let re = Regex::new(MAIN).unwrap();
        let caps = re.captures_iter(py_code);
        let mut body = String::new();

        for cap in caps {
            let content = cap.get(0).unwrap().as_str();
            body = format!("{}{}\n", body, content);
        }

        let mut body: Vec<Instruction> = Self::get_instructions(self, body);
        let type_ = Type::Int;
        let value = "0".to_string();

        body.push(
            Instruction::Return { type_, value }
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

    fn infer_param_types(self: &mut Code) {
        let mut called_funs = Vec::new();
        let mut fun_types = Vec::new();

        // find argument types
        for fun in self.functions.iter() {
            for instruction in fun.body.iter() {
                match instruction {
                    Instruction::CallFun { name, arguments } => {
                        let mut arg_types = Vec::new();
                        if NATIVE_FUNS.contains(&name.as_str()) {
                            continue;                           // exclude native functions
                        }
                        for argument in arguments.iter() {
                            arg_types.push(argument.type_.clone());
                        }
                        called_funs.push(name.to_string());     // store function name
                        fun_types.push(arg_types);              // store argument types
                    },
                    _ => {}
                };
            }
        }

        // update param types
        for fun in self.functions.iter_mut() {
            let fun_name = fun.name.to_string();
            if !called_funs.contains(&fun_name) {
                continue;                                       // exclude uncalled functions
            }
            for (call_index, call_name) in called_funs.iter().enumerate() {
                if &fun_name == call_name {
                    for (arg_index, mut param) in fun.params.iter_mut().enumerate() {
                        match param.type_ {
                            Type::Undefined => {
                                param.type_ = fun_types[call_index][arg_index].clone();
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn infer_return_types(self: &mut Code) {
        for fun in self.functions.iter_mut() {
            let mut there_is_return = false;
            for instruction in fun.body.iter() {
                match instruction {
                    Instruction::Return { type_, value: _ } => {
                        there_is_return = true;
                        fun.type_ = type_.clone();
                    },
                    _ => {}
                }
            }
            if !there_is_return {
                fun.type_ = Type::Void;
            }
        }
    }

    fn py2code(py_code: &str) -> Code {
        let re = Regex::new(DEC_FUN).unwrap();
        let caps = re.captures_iter(py_code);
        let mut code = Self::create_code();

        for cap in caps {
            let body = cap.get(1).unwrap().as_str();
            let body = Self::shift_code_left(body);
            let header = cap.get(0).unwrap().as_str();

            let type_: Type = Type::Undefined;
            let body: Vec<Instruction> = Self::get_instructions(&mut code, body);
            let (name, params): (String, Vec<Param>) = Self::get_header_info(header);

            code.functions.push(
                Function { type_, name, params, body }
            );
        }

        let main: Function = Self::get_main(&mut code, py_code);
        code.functions.push(main);

        Self::infer_param_types(&mut code);
        Self::infer_return_types(&mut code);

        code.libraries.sort();
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
                    let options = [
                        print::code2cpp(name, arguments),
                        input::code2cpp(name, arguments),
                        custom_fun::code2cpp(name, arguments)
                    ];
                    options.join("")
                },
                Instruction::CreateVar { type_, name, value } => {
                    declare::code2cpp(type_, name, value)
                },
                Instruction::Return { type_: _, value } => {
                    r#return::code2cpp(value)
                },
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
            if library.name == "iostream" || library.name == "string" {
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
