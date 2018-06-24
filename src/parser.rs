use std::fs;
use std::error::Error;

const TYPESCRIPT_FILE_PATH: &str = "./tests/fixtures/parsing.d.ts";

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<Argument>,
    pub return_type: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Argument {
    pub name: String,
    pub taipu: String,
}

fn is_function(s: &str) -> bool {
    match s {
        "function" => true,
        _ => false,
    }
}

pub fn read_typescript_files() -> Result<String, Box<Error>> {
    fs::read_to_string(TYPESCRIPT_FILE_PATH).map_err(|e| e.into())
}

named!(parse_one_arg<&str, Argument>,
    complete!(do_parse!(
        name: ws!(take_until_and_consume!(":")) >>
        taipu: ws!(alt_complete!(take_until_and_consume!(",") | take_until!(")"))) >>
        (Argument { name: name.to_string(), taipu: taipu.to_string() })
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

named!(parse_return_type<&str, Option<String>>,
    do_parse!(
        return_type: ws!(take_until!(";")) >>
        (match return_type {
            "void" => None,
            _ => None,
        })
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
    assert_eq!(parsed_function.return_type, None);
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
        taipu: "string".to_string()
    }]);
    assert_eq!(parsed_function.return_type, None);
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
        taipu: "string".to_string()
    }, Argument{
        name: "age".to_string(),
        taipu: "number".to_string()
    }]);
    assert_eq!(parsed_function.return_type, None);
}
