use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    String,
    Void,
    Vector(Box<Type>),
    Generic,
    Undefined
}

#[derive(Debug, Clone)]
pub struct Param {
    pub type_: Type,
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub type_: Type,
    pub value: Value
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    None
}

#[derive(Debug, Clone)]
pub enum Value {
    ConstValue(String),
    UseVar(String),
    CallFun { name: String, arguments: Vec<Argument> },
    Expression { operators: Vec<Operator>, values: Vec<Value> },
    None
}

#[derive(Debug)]
pub enum Instruction {
    CreateVar { type_: Type, name: String, value: Value },
    ReassignVar { type_: Type, name: String, value: Value },
    CallFun { name: String, arguments: Vec<Argument> },
    Loop { counter: String, start: Value, end: Value, content: Vec<Instruction> },
    Return { type_: Type, value: String }
}

#[derive(Debug)]
pub struct Function {
    pub type_: Type,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Instruction>
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Library {
    pub name: String
}

#[derive(Debug)]
pub struct Code {
    pub libraries: Vec<Library>,
    pub functions: Vec<Function>
}

pub struct Context (
    pub HashMap<String, Vec<Param>>
);
