use crate::py2cpp::types::{Type, Param, Instruction, Function, Code, Context};
use crate::py2cpp::constants::{RE_COMMENTS, RE_PARAMS_MULTILINE, RE_ARRAY_MULTILINE, RE_HEAD_DEC_FUN, RE_DEC_FUN, RE_PARAMS, RE_INSTRUCTIONS, RE_SHIFT_LEFT, RE_MAIN};
use crate::py2cpp::instructions::{print, custom_fun, declare, append, r#loop, r#return};
use crate::py2cpp::infer;
use std::process::Command;

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

    pub fn get_instructions(&mut self, fun_body: &mut Vec<Instruction>, context: &mut Context, body: String) -> Result<Vec<Instruction>, String> {
        let caps = RE_INSTRUCTIONS.captures_iter(&body);
        let mut body: Vec<Instruction> = Vec::new();
        let mut is_match = false;

        for cap in caps {
            let content = cap.get(1).unwrap().as_str();

            let results = [
                print::py2code(context, content, true),
                declare::py2code(context, content),
                custom_fun::py2code(context, content),
                append::py2code(context, fun_body, content),
                r#loop::py2code(self, &mut body, context, content),
                r#return::py2code(&body, context, content)
            ];

            is_match = false;

            for result in results {
                match result {
                    Ok(option) => {
                        match option {
                            Some((mut instructions, mut libraries)) => {
                                self.libraries.append(&mut libraries);
                                body.append(&mut instructions);
                                is_match = true;
                            },
                            None => {}
                        }
                    },
                    Err(error) => return Err(error)
                }
            }

            if !is_match { break; }

        }
        match is_match {
            true  => Ok(body),
            false => Err(format!("Comando no identificado"))
        }
    }

    fn get_main(&mut self, py_code: &str) -> Result<Function, String> {
        let caps = RE_MAIN.captures_iter(py_code);
        let mut body = String::new();
        let mut context = Context::get_fun_types(self);

        let mut main = Function {
            type_: Type::Int,
            name: "main".to_string(),
            params: Vec::new(),
            body: vec![]
        };

        let return_ = Instruction::Return {
            type_: Type::Int,
            value: "0".to_string()
        };

        for cap in caps {
            let content = cap.get(0).unwrap().as_str();
            body = format!("{}{}\n", body, content);
        }

        let result = self.get_instructions(&mut vec![], &mut context, body);
        match result {
            Ok(mut body) => {
                body.push(return_);
                main.body = body;
                Ok(main)
            },
            Err(error) => Err(error)
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

    fn py2code(py_code: &str) -> Result<Code, String> {
        let caps = RE_DEC_FUN.captures_iter(py_code);
        let mut code = Self::create_code();

        for cap in caps {
            let body = cap.get(1).unwrap().as_str();
            let body = Self::shift_code_left(body);
            let header = cap.get(0).unwrap().as_str();
            let mut context = Context::get_fun_types(&mut code);
            let (name, params): (String, Vec<Param>) = Self::get_header_info(header);

            for param in params.iter() {
                context.0.insert(param.name.to_string(), vec![param.clone()]);
            }

            let body: Result<Vec<Instruction>, String> = code.get_instructions(&mut vec![], &mut context, body);

            match body {
                Ok(mut body) => {
                    let type_: Type = infer::get_return_type(&mut body);
                    code.functions.push(
                        Function { type_, name, params, body }
                    );
                },
                Err(error) => return Err(error)
            }

        }

        let result: Result<Function, String> = code.get_main(py_code);
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        let main = result.ok().unwrap();
        code.functions.push(main);

        match infer::param_types(&mut code) {
            Ok(_) => {
                code.libraries.sort();
                code.libraries.dedup();         // remove duplicate libraries
                Ok(code)
            }
            Err(error) => Err(error)
        }

    }

    fn fun2cpp(function: &Function) -> String {
        // generate function header
        let type_ = function.type_.type2cpp();
        let mut there_are_generics = false;
        let mut header = format!("{} {}(", type_, function.name);

        for (index, param) in function.params.iter().enumerate() {
            if index > 0 {
                header.push_str(", ");
            }
            if param.type_ == Type::Generic {
                there_are_generics = true;
            }
            let type_ = param.type_.type2cpp();
            header = format!("{}{} {}", header, type_, param.name);
        }

        if there_are_generics {
            header = format!("template <typename T>\n\n{}", header);
        }
        let body = Instruction::insts2cpp(&function.body, 1);
        // generate function body
        format!("{}) {{\n{}}}\n", header, body)
    }

    fn code2cpp(self) -> String {
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

    pub fn filter(py_code: &str) -> String {
        let mut code = RE_COMMENTS.replace_all(py_code, "").to_string();

        loop {
            let cap = RE_PARAMS_MULTILINE.captures(&code);
            match cap {
                Some(some) => {
                    let params_multiline = some.get(0).unwrap().as_str();
                    let params_oneline = params_multiline.replace("\n", "");
                    code = code.replace(&params_multiline, &params_oneline).to_string();
                },
                None => break
            }
        }

        loop {
            let cap = RE_ARRAY_MULTILINE.captures(&code);
            match cap {
                Some(some) => {
                    let array_multiline = some.get(0).unwrap().as_str();
                    let array_oneline = array_multiline.replace("\n", "");
                    code = code.replace(&array_multiline, &array_oneline).to_string();
                },
                None => break
            }
        }

        code
    }

    pub fn transpile(py_code: &str, file_name: &str) -> Result<String, String> {
        let status = Command::new("python3").arg("-m").arg("py_compile").arg(file_name).status().ok().unwrap();
        let code = status.code().unwrap();

        if code == 1 {
            println!();
            return Err("Error de sintaxis".to_string());
        }

        let py_code = Self::filter(py_code);
        match Code::py2code(&py_code) {
            Ok(code) => {
                println!("{:?}", code);
                Ok(code.code2cpp())
            },
            Err(error) => Err(error)
        }
    }

}
