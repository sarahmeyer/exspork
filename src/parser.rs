use std::fs;
use std::error::Error;
use std::fmt;
use nom::{line_ending};

const TYPESCRIPT_FILE_PATH: &str = "./tests/fixtures/parsing.d.ts";

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<Argument>,
    pub return_type: Type,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut stringified_args = String::new();
        for arg in self.args.iter() {
            stringified_args.push_str(&arg.to_string());
            stringified_args.push('\n');
        }
        write!(f, "Function {} with args {} returns a {}!!",
            self.name,
            stringified_args,
            self.return_type,
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Argument {
    pub name: String,
    pub taipu: Type,
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Argument {} of type {}!", self.name, self.taipu)
    }
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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Boolean => write!(f, "boolean"),
            Type::Number => write!(f, "number"),
            Type::String => write!(f, "string"),
            Type::Array => write!(f, "array"),
            Type::Tuple => write!(f, "tuple"),
            Type::Enum => write!(f, "enum"),
            Type::Any => write!(f, "any"),
            Type::Void => write!(f, "void"),
            Type::Null => write!(f, "null"),
            Type::Undefined => write!(f, "undefined"),
            Type::Never => write!(f, "never"),
            Type::Object => write!(f, "object"),
            Type::Custom(ref taipu) => write!(f, "{}", &taipu.to_string()),
        }
    }
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

fn is_argument_delimiter(c: char) -> bool {
    match c {
        ',' |
        ')' => true,
        _ => false,
    }
}

fn is_comma(c: char) -> bool {
    match c {
        ',' => true,
        _ => false,
    }
}

named!(parse_one_arg<&str, Argument>,
    complete!(do_parse!(
        name: ws!(take_until_and_consume!(":")) >>
        taipu: ws!(take_till!(is_argument_delimiter)) >>
        (Argument { name: name.to_string(), taipu: to_type(taipu) })
    ))
);

named!(parse_arg_with_comma<&str, Argument>,
    complete!(do_parse!(
        arg: parse_one_arg >>
        ws!(tag!(",")) >>
        (arg)
    ))
);

named!(parse_args<&str, Vec<Argument>>,
    do_parse!(
        args: delimited!(
            tag!("("),
            opt!(many1!(alt_complete!(
                parse_arg_with_comma |
                parse_one_arg
            ))),
            tag!(")")
        ) >>
        (match args {
            Some(args) => args,
            None => Vec::new(),
        })
    )
);

named!(parse_return_type<&str, Type>,
    do_parse!(
        return_type: ws!(take_until!(";")) >>
        (to_type(return_type))
    )
);

named!(parse_function<&str, Function>,
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

named!(parse_definition_file<&str, Vec<Function>>,
    do_parse!(
        functions: many0!(parse_function) >>
        (functions)
    )
);

#[test]
fn parse_void_function() {
    let void_function_declaration = "
        export function greet(): void;

    ";

    let (_, parsed_function) = parse_function(void_function_declaration).unwrap();

    assert_eq!(&parsed_function.name, "greet");
    assert_eq!(parsed_function.args, Vec::new());
    assert_eq!(parsed_function.return_type, Type::Void);
}

#[test]
fn parse_void_function_with_arg() {
    let void_function_declaration = "
        export function greet(person: string): void;

    ";

    let (_, parsed_function) = parse_function(void_function_declaration).unwrap();

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

    let (_, parsed_function) = parse_function(void_function_declaration).unwrap();

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

    let (_, parsed_function) = parse_function(void_function_declaration).unwrap();

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

#[test]
fn parse_multiple_functions() {
    let multiple_functions = "
        export function greet(person: string): void;

        export function exponent(x: number, y: number): number;
        export function concat(first: array, second: array): array;

        export function isEqual(x: number, y: number): boolean;

    ";

    let (_, parsed_functions) = parse_definition_file(multiple_functions).unwrap();

    let first_parsed_function = &parsed_functions[0];
    assert_eq!(&first_parsed_function.name, "greet");
    assert_eq!(first_parsed_function.args, vec![Argument{
        name: "person".to_string(),
        taipu: Type::String,
    }]);
    assert_eq!(first_parsed_function.return_type, Type::Void);

    let second_parsed_function = &parsed_functions[1];
    assert_eq!(&second_parsed_function.name, "exponent");
    assert_eq!(second_parsed_function.args, vec![Argument{
        name: "x".to_string(),
        taipu: Type::Number,
    }, Argument{
        name: "y".to_string(),
        taipu: Type::Number,
    }]);
    assert_eq!(second_parsed_function.return_type, Type::Number);

    assert_eq!(parsed_functions.len(), 4);

}

// #[test]
// fn parse_invalid_input() {
//     let multiple_functions = "
//         export function greet(person: ): void;

//         export function  number;
//         export function , second: array): array;

//         export function isEqual(x: number, y: number): ;

//     ";

//     let (_, parsed_functions) = parse_definition_file(multiple_functions).unwrap();


// }
