use crate::py2cpp::types::{Value, Operator};
use crate::py2cpp::constants::{RE_VAL, RE_OPR};

impl Value {

    pub fn exp2value(content: &str) -> Value {
        let caps_val = RE_VAL.captures_iter(content);
        let caps_opr = RE_OPR.captures_iter(content);
        let mut operators = Vec::new();
        let mut values = Vec::new();

        for cap in caps_opr {
            let operator = cap.get(0).unwrap().as_str();
            operators.push(Operator::str2opr(operator));
        }
        for cap in caps_val {
            let content = cap.get(1).unwrap().as_str();
            values.push(Value::ConstValue(content.to_string()));
        }

        Value::Expression { operators, values }
    }

    pub fn exp2cpp(operators: &Vec<Operator>, values: &Vec<Value>) -> String{
        let mut result = String::new();
        for (index, value) in values.iter().enumerate() {
            result = match value {
                Value::ConstValue(value) => {
                    match index == 0 {
                        true => format!("{}{}", result, value),
                        false  => format!("{} {} {}", result, Operator::opr2cpp(&operators[index-1]), value)
                    }
                },
                _ => String::new()
            }
        }
        result
    }

}
