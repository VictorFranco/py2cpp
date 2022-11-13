use crate::py2cpp::types::Operator;

impl Operator {

    pub fn str2opr(operator: &str) -> Operator {
        match operator {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => Operator::None
        }
    }

    pub fn opr2cpp(&self) -> String {
        match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::None => "",
        }.to_string()
    }

}
