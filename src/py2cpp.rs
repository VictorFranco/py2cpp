use regex::Regex;
use crate::instructions::{print, input, custom_fun, declare, r#return};
use crate::constants::{HEAD_DEC_FUN, PARAMS, INSTRUCTIONS, MAIN, SHIFT_LEFT, DEC_FUN};
use crate::infer;

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    String,
    Void,
    Vector(Box<Type>),
    Undefined
}

pub fn type2cpp(type_: &Type) -> String {
    match type_ {
        Type::Vector(type_) => {
            format!("vector<{}>", type2cpp(type_))
        },
        _ => match type_ {
            Type::Int => "int",
            Type::String => "string",
            Type::Void => "void",
            Type::Undefined => "undefined",
            _ => ""
        }.to_string()
    }
}

#[derive(Debug)]
pub struct Param {
    pub type_: Type,
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub type_: Type,
    pub value: Value
}

#[derive(Debug, Clone)]
pub enum Value {
    ConstValue(String),
    UseVar(String),
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

pub fn instruc2value(instruction: &Instruction) -> Value {
    match instruction {
        Instruction::CallFun { name, arguments } => {
            let name = name.to_string();
            let arguments = arguments.to_vec();
            Value::CallFun { name, arguments }
        },
       _ => Value::None
    }
}

#[derive(Debug)]
pub struct Function {
    pub type_: Type,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Instruction>
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Library {
    name: String
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
    pub libraries: Vec<Library>,
    pub functions: Vec<Function>
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
                print::py2code(content, true),
                declare::py2code(&mut body, content),
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

        infer::return_types(&mut code);
        infer::param_types(&mut code);
        infer::var_types(&mut code);

        code.libraries.sort();
        code.libraries.dedup();         // remove duplicate libraries

        code
    }

    fn fun2cpp(function: &Function) -> String {

        // generate function header
        let type_ = type2cpp(&function.type_);
        let mut header = format!("{} {}(", type_, function.name);
        for (index, param) in function.params.iter().enumerate() {
            if index > 0 {
                header.push_str(", ");
            }
            let type_ = type2cpp(&param.type_);
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
                        custom_fun::code2cpp(name, arguments, true)
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
        for library in self.libraries.iter() {
            result = format!("{}#include <{}>\n", result, library.name);
        }
        result.push('\n');
        // add namespace
        if self.libraries.len() > 0 {
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
