use std::collections::HashMap;
use crate::py2cpp::types::{Type, Param, Argument, Instruction, Value, Code, Context};
use crate::py2cpp::constants::NATIVE_FUNS;

fn store_arg_types(name: &String, called_funs: &mut Vec<String>, fun_types: &mut Vec<Vec<Type>>, arguments: &Vec<Argument>) {
    let mut arg_types = Vec::new();
    if !NATIVE_FUNS.contains(&name.as_str()) {
        for argument in arguments.iter() {
            arg_types.push(argument.type_.clone());
            match &argument.value {
                Value::CallFun { name, arguments } => {
                    store_arg_types(&name, called_funs, fun_types, &arguments);
                },
                _ => {}
            }
        }
        called_funs.push(name.to_string());     // store function name
        fun_types.push(arg_types);              // store argument types
    }
}

pub fn param_types(code: &mut Code) {
    let mut called_funs = Vec::new();
    let mut fun_types = Vec::new();

    // find argument types
    for fun in code.functions.iter() {
        for instruction in fun.body.iter() {
            match instruction {
                Instruction::CallFun { name, arguments } => {
                    store_arg_types(name, &mut called_funs, &mut fun_types, arguments);
                },
                Instruction::CreateVar { type_: _, name: _, value } => {
                    match value {
                        Value::CallFun { name, arguments } => {
                            store_arg_types(name, &mut called_funs, &mut fun_types, arguments);
                        },
                        _ => {}
                    }
                }
                _ => {}
            };
        }
    }

    // update param types
    for fun in code.functions.iter_mut() {
        let fun_name = fun.name.to_string();
        if !called_funs.contains(&fun_name) {
            continue;                                       // exclude uncalled functions
        }
        for (call_index, call_name) in called_funs.iter().enumerate() {
            if &fun_name == call_name {
                for (arg_index, mut param) in fun.params.iter_mut().enumerate() {
                    let arg_type = fun_types[call_index][arg_index].clone();  // argument type
                    param.type_ = match &param.type_ {
                        Type::Undefined => arg_type,
                        param_type if param_type != &arg_type => Type::Generic,
                        _ => param.type_.clone()
                    }
                }
            }
        }
    }
}

pub fn get_return_type(body: &mut Vec<Instruction>) -> Type {
    for instruction in body.iter() {
        match instruction {
            Instruction::Return { type_, value: _ } => {
                return type_.clone();
            },
            _ => {}
        }
    }
    Type::Void
}

pub fn get_fun_types(code: &mut Code) -> Context {
    let mut return_types = HashMap::new();
    for fun in code.functions.iter() {
        let param = Param {
            type_: fun.type_.clone(),
            name: fun.name.to_string()
        };
        return_types.insert(
            fun.name.to_string(),
            vec![param]
        );
    }
    Context(return_types)
}

pub fn get_type(name: &str, context: &Context) -> Type {
    let Param { type_, name: _ } = context.0.get(name).unwrap().last().unwrap().clone();
    type_
}

pub fn get_var_type(var_name: &str, body: &Vec<Instruction>) -> Type {
    let mut result = Type::Undefined;
    for instruction in body.iter() {
        match instruction {
            Instruction::CreateVar { type_, name, value: _ } => {
                if var_name == name {
                    result = type_.clone();
                }
            },
            _ => {}
        }
    }
    result
}
