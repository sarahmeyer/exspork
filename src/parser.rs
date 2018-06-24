use std::fs;
use std::error::Error;

const TYPESCRIPT_FILE_PATH: &str = "./tests/fixtures/parsing.d.ts";

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<Argument>,
    pub return_type: Type,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Argument {
    pub name: String,
    pub taipu: Type,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Boolean,
    Number,
    String,
    Array,
    Tuple,
    Enum,
    Any,
    Void,
    Null,
    Undefined,
    Never,
    Object,
    Custom(String),
}

fn to_type(s: &str) -> Type {
    match s {
        "boolean" => Type::Boolean,
        "number" => Type::Number,
        "string" => Type::String,
        "array" => Type::Array,
        "tuple" => Type::Tuple,
        "enum" => Type::Enum,
        "any" => Type::Any,
        "void" => Type::Void,
        "null" => Type::Null,
        "undefined" => Type::Undefined,
        "never" => Type::Never,
        "object" => Type::Object,
        custom => Type::Custom(custom.to_string()),
    }
}

pub fn read_typescript_files() -> Result<String, Box<Error>> {
    fs::read_to_string(TYPESCRIPT_FILE_PATH).map_err(|e| e.into())
}

named!(parse_one_arg<&str, Argument>,
    complete!(do_parse!(
        name: ws!(take_until_and_consume!(":")) >>
        taipu: ws!(alt_complete!(take_until_and_consume!(",") | take_until!(")"))) >>
        (Argument { name: name.to_string(), taipu: to_type(taipu) })
    ))
);

named!(parse_args<&str, Vec<Argument>>,
    do_parse!(
        tag!("(") >>
        args: many0!(parse_one_arg) >>
        tag!(")") >>
        (args)
    )
);

named!(parse_return_type<&str, Type>,
    do_parse!(
        return_type: ws!(take_until!(";")) >>
        (to_type(return_type))
    )
);

named!(function_declaration<&str, Function>,
    do_parse!(
        ws!(tag!("export")) >>
        ws!(tag!("function")) >>
        name: ws!(take_until!("(")) >>
        args: parse_args >>
        tag!(":") >>
        return_type: parse_return_type >>
        (Function { name: name.to_string(), args, return_type })
    )
);

#[test]
fn parse_void_function() {
    let void_function_declaration = "
        export function greet(): void;

    ";

    let (_, parsed_function) = function_declaration(void_function_declaration).unwrap();

    assert_eq!(&parsed_function.name, "greet");
    assert_eq!(parsed_function.args, Vec::new());
    assert_eq!(parsed_function.return_type, Type::Void);
}

#[test]
fn parse_void_function_with_arg() {
    let void_function_declaration = "
        export function greet(person: string): void;

    ";

    let (_, parsed_function) = function_declaration(void_function_declaration).unwrap();

    assert_eq!(&parsed_function.name, "greet");
    assert_eq!(parsed_function.args, vec![Argument{
        name: "person".to_string(),
        taipu: Type::String,
    }]);
    assert_eq!(parsed_function.return_type, Type::Void);
}

#[test]
fn parse_void_function_with_multiple_args() {
    let void_function_declaration = "
        export function greet(person: string, age: number): void;

    ";

    let (_, parsed_function) = function_declaration(void_function_declaration).unwrap();

    assert_eq!(&parsed_function.name, "greet");
    assert_eq!(parsed_function.args, vec![Argument{
        name: "person".to_string(),
        taipu: Type::String,
    }, Argument{
        name: "age".to_string(),
        taipu: Type::Number,
    }]);
    assert_eq!(parsed_function.return_type, Type::Void);
}

#[test]
fn parse_returning_function_with_multiple_args() {
    let void_function_declaration = "
        export function exponent(x: number, y: number): number;

    ";

    let (_, parsed_function) = function_declaration(void_function_declaration).unwrap();

    assert_eq!(&parsed_function.name, "exponent");
    assert_eq!(parsed_function.args, vec![Argument{
        name: "x".to_string(),
        taipu: Type::Number,
    }, Argument{
        name: "y".to_string(),
        taipu: Type::Number,
    }]);
    assert_eq!(parsed_function.return_type, Type::Number);
}
