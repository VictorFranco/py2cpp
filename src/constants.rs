// functions
pub const HEAD_DEC_FUN: &str = r"(?m)def\s([a-zA-Z][a-zA-Z_-]*)\(((([a-zA-Z][a-zA-Z0-9]*),?)*)\):";
pub const DEC_FUN: &str = r"(?m)def\s[a-zA-Z][a-zA-Z_-]*\(.*\):\n((\s{4,}.*\n)*)";
pub const PARAMS: &str = r"[a-zA-Z][a-zA-Z0-9]*";
pub const INSTRUCTIONS: &str = r"(?m)(.*)\n";
pub const SHIFT_LEFT: &str = r"(?m)\s{4}(.*)\n";
pub const MAIN: &str = r"(?m)^\S{4,}.*$";
// data types
pub const INTEGER: &str = r"^[+-]?\s*(\d+)$";
pub const STRING: &str = r##"^"[a-zA-Z0-9: ]*"$"##;
pub const VECTOR: &str = r"^\[\]$";
pub const VARIABLE: &str = r"^[a-zA-Z][a-zA-Z0-9]*$";
// instructions
pub const PRINT: &str = r##"^print\((.*)\)[^"]*$"##;
pub const MESSAGES: &str = r##"("[ a-zA-Z0-9: ]+"|[a-zA-Z][a-zA-Z0-9]+),?"##;
pub const INPUT: &str = r##"^input\((.*)\)$"##;
pub const CUSTOM_FUN: &str = r##"^([a-zA-Z][a-zA-Z0-9]*)\((.*)\)[^"]*$"##;
pub const ARGUMENTS: &str = r##"([+-]?\s*\d+|"[ a-zA-Z0-9: ]+"|[a-zA-Z][a-zA-Z0-9]*(\(.*\))?),?"##;
pub const DECLARE: &str = r##"(?m)^([a-zA-Z][a-zA-Z0-9]*)\s*=\s*(\d+|"[a-zA-Z0-9: ]*"|\[\]|([a-zA-Z][a-zA-Z0-9]*)\(.*\)?)$"##;
pub const INT: &str = r##"^int\((.*)\)$"##;
pub const RETURN: &str = r"^return (.*)$";

pub const NATIVE_FUNS: [&str; 3] = ["print", "input", "int"];
