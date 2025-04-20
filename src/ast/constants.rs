use phf::phf_map;

pub static OPERATOR_TO_FUNCTION_NAME_MAP: phf::Map<&str, &str> = phf_map! (
    "+" => "plus",
    "-" => "minus",
    "*" => "multiply",
    "/" => "divide",
);