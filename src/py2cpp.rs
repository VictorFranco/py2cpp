use regex::Regex;

// head of declared function
const HEAD_DEC_FUN:&str = r"def\s([a-zA-Z][a-zA-Z_-]*)\(([a-zA-Z][a-zA-Z0-9]*),?([a-zA-Z][a-zA-Z0-9]*)*\):";

// declared function with body
const DEC_FUN:&str = r"def\s[a-zA-Z][a-zA-Z_-]*\(.*\):(\n\s{4}.*)*";

// return
const RETURN:&str = r"return .*";

#[derive(Debug)]
struct Data {
    name: String,
    type_: String
}

#[derive(Debug)]
struct Function {
    name: String,
    type_: String,
    params: Vec<Data>,
}

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

fn py2vec(py_code: &str) -> Vec<Function> {
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

fn fun2cpp(dec_fun: &Function) -> String {
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

fn vec2cpp(functions: Vec<Function>) -> String {
    let mut result = String::new();
    for function in functions.iter() {
        result = format!("{}{}\n", result, fun2cpp(function));
    }
    result
}

pub fn transpile(py_code: &str) -> String {
    let functions: Vec<Function> = py2vec(py_code);
    println!("{:?}", functions);

    vec2cpp(functions)
}
