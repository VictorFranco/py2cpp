use crate::py2cpp::types::{Instruction, Value};
use crate::py2cpp::instructions::{print, input, custom_fun, declare, append, r#loop, r#return};

impl Instruction {

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
                Instruction::CreateVar { type_, name, value } => {
                    declare::code2cpp(type_, name, value, true)
                },
                Instruction::ReassignVar { type_, name, value } => {
                    declare::code2cpp(type_, name, value, false)
                },
                Instruction::CallFun { name, arguments } => {
                    let options = [
                        print::code2cpp(name, arguments),
                        input::code2cpp(name, arguments),
                        custom_fun::code2cpp(name, arguments, true),
                        append::code2cpp(name, arguments)
                    ];
                    options.join("")
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

}
