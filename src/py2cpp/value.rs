use crate::py2cpp::types::{Operator, Value, Context};
use crate::py2cpp::constants::{RE_VAL, RE_OPR, RE_AT, RE_INT, RE_VAR};
use crate::py2cpp::instructions::{custom_fun, int, len, at};

impl Value {

    pub fn exp2value(context: &mut Context, content: &str) -> Result<Value, String> {
        let caps_val = RE_VAL.captures_iter(content);
        let caps_opr = RE_OPR.captures_iter(content);
        let mut operators = Vec::new();
        let mut values = Vec::new();

        for cap in caps_opr {
            let operator = cap.get(0).unwrap().as_str();
            operators.push(Operator::str2opr(operator));
        }
        for cap in caps_val {
            let content = cap.get(1).unwrap().as_str().to_string();
            let result = match content.as_str() {
                text if RE_INT.is_match(text) => Ok(Value::ConstValue(content)),
                text if RE_VAR.is_match(text) => {
                    let result = context.get_type(text);
                    match result {
                        Ok(_) => Ok(Value::UseVar(content)),
                        Err(err) => Err(err)
                    }
                },
                text if RE_AT.is_match(text) => {
                    let result = at::py2code(context, text);
                    match result {
                        Ok(option) => {
                            match option {
                                Some((at_instructions, _at_libraries)) => {
                                    Ok(at_instructions[0].inst2value())
                                },
                                None => Ok(Value::None)
                            }
                        },
                        Err(error) => Err(error)
                    }
                },
                _ => Err("Valor no identificado".to_string())
            };
            match result {
                Ok(value) => values.push(value),
                Err(err) => return Err(err)
            }
        }

        Ok(Value::Expression { operators, values })
    }

    pub fn exp2cpp(operators: &Vec<Operator>, values: &Vec<Value>) -> String{
        let mut result = String::new();
        for (index, value) in values.iter().enumerate() {
            let value = match value {
                Value::ConstValue(value) | Value::UseVar(value) => {
                    value.to_string()
                },
                Value::CallFun { name, arguments } => {
                    match name.as_str() {
                        "int" => int::code2cpp(&arguments[0]),
                        "len" => len::code2cpp(&arguments[0]),
                        "at"  => at::code2cpp(name, arguments),
                        _ => custom_fun::code2cpp(name, arguments, false)
                    }
                },
                _ => String::new()
            };
            result = match index == 0 {
                true  => format!("{}{}", result, value),
                false => format!("{} {} {}", result, operators[index-1].opr2cpp(), value)
            };
        }
        result
    }

}
