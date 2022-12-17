use regex::Regex;
// filters
const COMMENTS: &str = r"(?m)\s*(#.*)?$";
const PARAMS_MULTILINE: &str = r"(?m)\([^)]*\n[^)]*\)";
const ARRAY_MULTILINE: &str = r"(?m)\[[^\]]*\n[^\]]*\]";
const MULTILINE: &str = r"(?m)\n";
// functions
const HEAD_DEC_FUN: &str = r"(?m)def\s([a-zA-Z][a-zA-Z_-]*)\(((\s*([a-zA-Z][a-zA-Z0-9]*)\s*,?)*)\)\s*:";
const DEC_FUN: &str = r"(?m)def\s[a-zA-Z][a-zA-Z_-]*\(.*\)\s*:\n((\s{4,}.*\n)*)";
const PARAMS: &str = r"[a-zA-Z][a-zA-Z0-9]*";
const INSTRUCTIONS: &str = r"(?m)(for.*\n((\s{4,}.*\n?)*)|(.*))\n";
const SHIFT_LEFT: &str = r"(?m)\s{4}(.*)\n?";
const MAIN: &str = r"(?m)^(\S{1}|\S{2}|[^d\s][^e][^f]\S.*)$";
// data types
const INTEGER: &str = r"^[+-]?\s*(\d+)$";
const STRING: &str = r##"^"[a-zA-Z0-9: ]*"$"##;
const VECTOR: &str = r"^\[\s*\]$";
const VARIABLE: &str = r"^[a-zA-Z][a-zA-Z0-9]*$";
// instructions
const PRINT: &str = r##"^print\s*\((.*)\)[^"]*$"##;
const MESSAGES: &str = r##"("[ a-zA-Z0-9: ]+"|[a-zA-Z][a-zA-Z0-9]*),?"##;
const INPUT: &str = r##"input\s*\(\s*(.*)\s*\)"##;
const FUNCTION: &str = r##"^([a-zA-Z][a-zA-Z0-9]*)\s*\((.*)\)[^"]*$"##;
const ARGUMENTS: &str = r##"([+-]?\s*\d+|"[ a-zA-Z0-9: ]+"|[a-zA-Z][a-zA-Z0-9]*(\(.*\))?),?"##;
const DECLARE: &str = r##"(?m)^([a-zA-Z][a-zA-Z0-9]*)\s*=\s*(.*)$"##;
const EXPRESSION: &str = r##"^((\d+|[a-zA-Z][a-zA-Z0-9]*\s*(\[.*\])?)\s*[+\-/*])+\s*(\d+|[a-zA-Z][a-zA-Z0-9]*\s*(\[.*\])?)$"##;
const VALUE: &str = r##"\s*(\d+|[a-zA-Z][a-zA-Z0-9]*(\s*\[.*\])?)\s*"##;
const OPERATOR: &str = r##"[+\-/*]"##;
const INT_FUN: &str = r##"^int\s*\(\s*(.*)\s*\)$"##;
const LOOP: &str = r"(?m)for\s*([a-zA-Z][a-zA-Z0-9]*)\s*in\s*range\s*\(\s*(\S*)\s*,\s*(\S*|.*)\s*\)\s*:\n((.*\n?)*)";
const LEN: &str = r"^\s*len\s*\(\s*(\S*)\s*\)\s*";
const AT: &str = r"^([a-zA-Z][a-zA-Z0-9]*)\s*\[\s*(\d+|[a-zA-Z][a-zA-Z0-9]*)\s*\]$";
const APPEND: &str = r"^([a-zA-Z][a-zA-Z0-9]*)\.append\(\s*(\S*)\s*\)\s*$";
const RETURN: &str = r"^return\s*(\S*)$";

pub const NATIVE_FUNS: [&str; 6] = ["print", "input", "int", "len", "append", "at"];

lazy_static! {
    pub static ref RE_COMMENTS: Regex = Regex::new(COMMENTS).unwrap();
    pub static ref RE_PARAMS_MULTILINE: Regex = Regex::new(PARAMS_MULTILINE).unwrap();
    pub static ref RE_ARRAY_MULTILINE: Regex = Regex::new(ARRAY_MULTILINE).unwrap();
    pub static ref RE_MULTILINE: Regex = Regex::new(MULTILINE).unwrap();

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
    pub static ref RE_FUN: Regex = Regex::new(FUNCTION).unwrap();
    pub static ref RE_ARGS: Regex = Regex::new(ARGUMENTS).unwrap();
    pub static ref RE_DEC: Regex = Regex::new(DECLARE).unwrap();
    pub static ref RE_EXP: Regex = Regex::new(EXPRESSION).unwrap();
    pub static ref RE_VAL: Regex = Regex::new(VALUE).unwrap();
    pub static ref RE_OPR: Regex = Regex::new(OPERATOR).unwrap();
    pub static ref RE_INT_FUN: Regex = Regex::new(INT_FUN).unwrap();
    pub static ref RE_LOOP: Regex = Regex::new(LOOP).unwrap();
    pub static ref RE_LEN: Regex = Regex::new(LEN).unwrap();
    pub static ref RE_AT: Regex = Regex::new(AT).unwrap();
    pub static ref RE_APPEND: Regex = Regex::new(APPEND).unwrap();
    pub static ref RE_RETURN: Regex = Regex::new(RETURN).unwrap();
}
