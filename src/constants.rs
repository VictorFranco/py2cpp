use regex::Regex;
// functions
const HEAD_DEC_FUN: &str = r"(?m)def\s([a-zA-Z][a-zA-Z_-]*)\(((([a-zA-Z][a-zA-Z0-9]*),?)*)\):";
const DEC_FUN: &str = r"(?m)def\s[a-zA-Z][a-zA-Z_-]*\(.*\):\n((\s{4,}.*\n)*)";
const PARAMS: &str = r"[a-zA-Z][a-zA-Z0-9]*";
const INSTRUCTIONS: &str = r"(?m)(.*)\n";
const SHIFT_LEFT: &str = r"(?m)\s{4}(.*)\n";
const MAIN: &str = r"(?m)^\S{4,}.*$";
// data types
const INTEGER: &str = r"^[+-]?\s*(\d+)$";
const STRING: &str = r##"^"[a-zA-Z0-9: ]*"$"##;
const VECTOR: &str = r"^\[\]$";
const VARIABLE: &str = r"^[a-zA-Z][a-zA-Z0-9]*$";
// instructions
const PRINT: &str = r##"^print\((.*)\)[^"]*$"##;
const MESSAGES: &str = r##"("[ a-zA-Z0-9: ]+"|[a-zA-Z][a-zA-Z0-9]+),?"##;
const INPUT: &str = r##"^input\((.*)\)$"##;
const CUSTOM_FUN: &str = r##"^([a-zA-Z][a-zA-Z0-9]*)\((.*)\)[^"]*$"##;
const ARGUMENTS: &str = r##"([+-]?\s*\d+|"[ a-zA-Z0-9: ]+"|[a-zA-Z][a-zA-Z0-9]*(\(.*\))?),?"##;
const DECLARE: &str = r##"(?m)^([a-zA-Z][a-zA-Z0-9]*)\s*=\s*(\d+|"[a-zA-Z0-9: ]*"|\[\]|([a-zA-Z][a-zA-Z0-9]*)(\(.*\))?)$"##;
const INT_FUN: &str = r##"^int\((.*)\)$"##;
const LOOP: &str = r"^for ([a-zA-Z][a-zA-Z0-9]*) in range\(\s*(.*)\s*,\s*(.*)\s*\):$";
const LEN: &str = r"^len\((.*)\)$";
const RETURN: &str = r"^return (.*)$";

pub const NATIVE_FUNS: [&str; 4] = ["print", "input", "int", "len"];

lazy_static! {
    pub static ref RE_HEAD_DEC_FUN: Regex = Regex::new(HEAD_DEC_FUN).unwrap();
    pub static ref RE_DEC_FUN: Regex = Regex::new(DEC_FUN).unwrap();
    pub static ref RE_PARAMS: Regex = Regex::new(PARAMS).unwrap();
    pub static ref RE_INSTRUCTIONS: Regex = Regex::new(INSTRUCTIONS).unwrap();
    pub static ref RE_SHIFT_LEFT: Regex = Regex::new(SHIFT_LEFT).unwrap();
    pub static ref RE_MAIN: Regex = Regex::new(MAIN).unwrap();

    pub static ref RE_INT: Regex = Regex::new(INTEGER).unwrap();
    pub static ref RE_STR: Regex = Regex::new(STRING).unwrap();
    pub static ref RE_VEC: Regex = Regex::new(VECTOR).unwrap();
    pub static ref RE_VAR: Regex = Regex::new(VARIABLE).unwrap();

    pub static ref RE_PRINT: Regex = Regex::new(PRINT).unwrap();
    pub static ref RE_MSGS: Regex = Regex::new(MESSAGES).unwrap();
    pub static ref RE_INPUT: Regex = Regex::new(INPUT).unwrap();
    pub static ref RE_FUN: Regex = Regex::new(CUSTOM_FUN).unwrap();
    pub static ref RE_ARGS: Regex = Regex::new(ARGUMENTS).unwrap();
    pub static ref RE_DEC: Regex = Regex::new(DECLARE).unwrap();
    pub static ref RE_INT_FUN: Regex = Regex::new(INT_FUN).unwrap();
    pub static ref RE_LOOP: Regex = Regex::new(LOOP).unwrap();
    pub static ref RE_LEN: Regex = Regex::new(LEN).unwrap();
    pub static ref RE_RETURN: Regex = Regex::new(RETURN).unwrap();
}
