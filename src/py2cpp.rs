use regex::Regex;

// head of declared function
const HEAD_DEC_FUN: &str = r"(?m)def\s([a-zA-Z][a-zA-Z_-]*)\(([a-zA-Z][a-zA-Z0-9]*),?([a-zA-Z][a-zA-Z0-9]*)*\):";

// declared function with body
const DEC_FUN: &str = r"(?m)def\s[a-zA-Z][a-zA-Z_-]*\(.*\):\n((\s{4,}.*\n)*)";

const INSTRUCTIONS: &str = r"(?m)\s{4,}(.*)\n";

const RETURN: &str = r"return .*";

const MAIN: &str = r"(?m)^\S{4,}.*$";

#[derive(Debug)]
struct Data {
    name: String,
    type_: String
}

#[derive(Debug)]
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
            functions: Vec::new()
        }
    }

    fn get_header(cap: &regex::Captures) -> (String, Vec<Data>) {
        let dec_fun = cap.get(0).unwrap().as_str();
        let re = Regex::new(HEAD_DEC_FUN).unwrap();
        let cap = re.captures(dec_fun).unwrap();
        let name = cap.get(1).unwrap().as_str().to_string();
        let mut params = Vec::new();

        for i in 2..cap.len() {
            match cap.get(i) {
                Some(data) => params.push(
                    Data {
                        name: data.as_str().to_string(),
                        type_: "undefined".to_string()
                    }
                ),
                None => {}
            }
        }
        (name, params)
    }

    fn get_return_type(cap: &regex::Captures) -> String {
        let dec_fun = cap.get(0).unwrap().as_str();
        let re = Regex::new(RETURN).unwrap();

        if !re.is_match(dec_fun) {      // if the function has no return statement
            return "void".to_string();  // then it is void type
        }
        else {
            return "undefined".to_string();
        }
    }

    fn get_body(cap: &regex::Captures) -> Vec<Instruction> {
        let body = cap.get(1).unwrap().as_str();        // extract function body
        let re = Regex::new(INSTRUCTIONS).unwrap();
        let caps = re.captures_iter(body);
        let mut instructions: Vec<Instruction> = Vec::new();

        for cap in caps {
            let content = cap.get(1).unwrap().as_str().to_string();
            instructions.push(
                Instruction { content }                 // save each instruction
            );
        }
        instructions
    }

    fn get_main(py_code: &str) -> Function {
        let re = Regex::new(MAIN).unwrap();
        let caps = re.captures_iter(py_code);
        let mut body: Vec<Instruction> = Vec::new();

        for cap in caps {
            let content = cap.get(0).unwrap().as_str().to_string();
            body.push(
                Instruction { content }
            );
        }
        Function {
            name: "main".to_string(),
            type_: "void".to_string(),
            params: Vec::new(),
            body
        }
    }

    fn py2code(py_code: &str) -> Code {
        let re = Regex::new(DEC_FUN).unwrap();
        let caps = re.captures_iter(py_code);
        let mut code = Self::create_code();

        for cap in caps {
            let type_: String = Self::get_return_type(&cap);   // get function type
            let body: Vec<Instruction> = Self::get_body(&cap); // get function body
            let (name, params): (String, Vec<Data>) = Self::get_header(&cap);

            code.functions.push(
                Function { name, type_, params, body }
            );
        }

        let main: Function = Self::get_main(py_code);
        code.functions.push(main);

        code
    }

    fn fun2cpp(function: &Function) -> String {
        // generate function header
        let mut result = format!("{} {}(", function.type_, function.name);
        for (index, param) in function.params.iter().enumerate() {
            if index > 0 {
                result.push_str(", ");
            }
            result = format!("{}{} {}", result, param.type_, param.name);
        }
        // generate function body
        let mut body = String::new();
        for instruction in &function.body {
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
