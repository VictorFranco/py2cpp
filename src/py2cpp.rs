use regex::Regex;

// head of declared function
const HEAD_DEC_FUN:&str = r"def\s([a-zA-Z][a-zA-Z_-]*)\(([a-zA-Z][a-zA-Z0-9]*),?([a-zA-Z][a-zA-Z0-9]*)*\):";

// declared function with body
const DEC_FUN:&str = r"def\s[a-zA-Z][a-zA-Z_-]*\(.*\):\n((\s{4,}.*\n)*)";

// instructions
const INSTRUCTIONS:&str = r"\s{4,}(.*)\n";

// return
const RETURN:&str = r"return .*";

#[derive(Debug)]
struct Data {
    name: String,
    type_: String
}

#[derive(Debug, Clone)]
struct Instruction {
    content: String
}

#[derive(Debug)]
struct Function {
    name: String,
    type_: String,
    params: Vec<Data>,
    body: Vec<Instruction>
}

#[allow(unused)]
#[derive(Debug)]
struct Library {
    name: String
}

#[allow(unused)]
#[derive(Debug)]
struct Code {
    libraries: Vec<Library>,
    functions: Vec<Function>
}

impl Code {

    fn create_code() -> Code {
        Code {
            libraries: Vec::new(),
            functions: Vec::new(),
        }
    }

    fn get_return_types(py_code: &str) -> Vec<String> {
        let re = Regex::new(DEC_FUN).unwrap();
        let caps = re.captures_iter(py_code);
        let mut returns = Vec::new();

        for cap in caps {
            let dec_fun = cap.get(0).unwrap().as_str();
            let re = Regex::new(RETURN).unwrap();
            if !re.is_match(dec_fun) {        // if the function has no return statement
                returns.push("void".to_string());   // then it is void type
            }
            else {
                returns.push("undefined".to_string());
            }
        }
        returns
    }

    fn get_bodies(py_code: &str) -> Vec<Vec<Instruction>> {
        let re = Regex::new(DEC_FUN).unwrap();
        let caps = re.captures_iter(py_code);
        let mut returns = Vec::new();
        let re = Regex::new(INSTRUCTIONS).unwrap();

        for cap in caps {
            let body = cap.get(1).unwrap().as_str();             // extract function body
            let mut instructions:Vec<Instruction> = Vec::new();
            let caps = re.captures_iter(body);

            for cap in caps {
                let content = cap.get(1).unwrap().as_str().to_string();
                instructions.push(
                    Instruction { content }                            // save each instruction
                );
            }
            returns.push(instructions);
        }
        returns
    }

    fn py2code(py_code: &str) -> Code {
        let re = Regex::new(HEAD_DEC_FUN).unwrap();
        let caps = re.captures_iter(py_code).enumerate();
        let mut code = Self::create_code();

        let types: Vec<String> = Self::get_return_types(py_code);      // get each function type
        let bodies: Vec<Vec<Instruction>> = Self::get_bodies(py_code); // get each function body

        for (index, cap) in caps {
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

            code.functions.push(
                Function {
                    name: cap[1].to_string(),
                    type_: types[index].to_string(),
                    params,
                    body: bodies[index].clone()
                }
            );
        }
        code
    }

    fn fun2cpp(dec_fun: &Function) -> String {
        // generate function header
        let mut result = format!("{} {} (", dec_fun.type_, dec_fun.name);
        for (index, param) in dec_fun.params.iter().enumerate() {
            if index > 0 {
                result.push_str(", ");
            }
            result = format!("{}{} {}", result, param.type_, param.name);
        }
        // generate function body
        let mut body = String::new();
        for instruction in &dec_fun.body {
            body = format!("{}{}\n", body, instruction.content);
        }
        result = format!("{}) {{\n{}}}\n", result, body);
        result
    }

    fn code2cpp(self: Code) -> String {
        let mut result = String::new();
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
