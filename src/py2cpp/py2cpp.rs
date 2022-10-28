use crate::py2cpp::types::{Type, Param, Instruction, Value, Function, Library, Code};
use crate::py2cpp::constants::{RE_HEAD_DEC_FUN, RE_DEC_FUN, RE_PARAMS, RE_INSTRUCTIONS, RE_SHIFT_LEFT, RE_MAIN};
use crate::py2cpp::instructions::{print, input, custom_fun, declare, r#loop, r#return};
use crate::py2cpp::infer;

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
            Type::Generic => "T",
            _ => ""
        }.to_string()
    }
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

pub fn insts2cpp(instructions: &Vec<Instruction>, tabs: u32) -> String {
    let mut result = String::new();
    for instruction in instructions {
        for _ in 0..tabs {
            result.push_str( "    ");
        }
        let cpp_instruction = match instruction {
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
            Instruction::Loop { counter, start, end, content } => {
                r#loop::code2cpp(counter, start, end, content, tabs + 1)
            },
            Instruction::Return { type_: _, value } => {
                r#return::code2cpp(value)
            }
        };
        result = format!("{}{}\n", result, cpp_instruction);
    }
    result
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

impl Code {

    fn create_code() -> Code {
        Code {
            libraries: Vec::new(),
            functions: Vec::new()
        }
    }

    fn get_header_info(header: &str) -> (String, Vec<Param>) {
        let cap = RE_HEAD_DEC_FUN.captures(header).unwrap();
        let name = cap.get(1).unwrap().as_str().to_string();    // get function name

        let params = cap.get(2).unwrap().as_str();
        let caps = RE_PARAMS.captures_iter(&params);
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

    pub fn get_instructions(self: &mut Code, body: String) -> Vec<Instruction> {
        let caps = RE_INSTRUCTIONS.captures_iter(&body);
        let mut body: Vec<Instruction> = Vec::new();
        let fun_types = infer::get_fun_types(self);

        for cap in caps {
            let content = cap.get(1).unwrap().as_str();
            let results = [
                print::py2code(content, true),
                declare::py2code(&mut body, &fun_types, content),
                custom_fun::py2code(&mut body, &fun_types, content),
                r#loop::py2code(self, &mut body, &fun_types, content),
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
        let caps = RE_MAIN.captures_iter(py_code);
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

    pub fn shift_code_left(body: &str) -> String {
        let caps = RE_SHIFT_LEFT.captures_iter(&body);
        let mut body = String::new();

        for cap in caps {
            let content = cap.get(1).unwrap().as_str();
            body = format!("{}{}\n", body, content);
        }

        body
    }

    fn py2code(py_code: &str) -> Code {
        let caps = RE_DEC_FUN.captures_iter(py_code);
        let mut code = Self::create_code();

        for cap in caps {
            let body = cap.get(1).unwrap().as_str();
            let body = Self::shift_code_left(body);
            let header = cap.get(0).unwrap().as_str();

            let mut body: Vec<Instruction> = Self::get_instructions(&mut code, body);
            let type_: Type = infer::get_return_type(&mut body);
            let (name, params): (String, Vec<Param>) = Self::get_header_info(header);

            code.functions.push(
                Function { type_, name, params, body }
            );
        }

        let main: Function = Self::get_main(&mut code, py_code);
        code.functions.push(main);

        infer::param_types(&mut code);

        code.libraries.sort();
        code.libraries.dedup();         // remove duplicate libraries

        code
    }

    fn fun2cpp(function: &Function) -> String {
        // generate function header
        let type_ = type2cpp(&function.type_);
        let mut there_are_generics = false;
        let mut header = format!("{} {}(", type_, function.name);
        for (index, param) in function.params.iter().enumerate() {
            if index > 0 {
                header.push_str(", ");
            }
            if param.type_ == Type::Generic {
                there_are_generics = true;
            }
            let type_ = type2cpp(&param.type_);
            header = format!("{}{} {}", header, type_, param.name);
        }
        if there_are_generics {
            header = format!("template <typename T>\n\n{}", header);
        }
        let body = insts2cpp(&function.body, 1);
        // generate function body
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
        result.pop();       // remove last blank line
        result
    }

}

pub fn transpile(py_code: &str) -> String {
    let code: Code = Code::py2code(py_code);
    println!("{:?}", code);

    Code::code2cpp(code)
}
