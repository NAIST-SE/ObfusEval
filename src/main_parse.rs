use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha0, alpha1, multispace0},
    combinator::{map, opt, rest_len},
    sequence::{delimited, tuple},
    Err, IResult,
};

// #[derive(Debug, PartialEq)]
// enum Token {
//     IncludeLibrary(String),
//     Prototype(String, Token::StorageClass),
//     Struct(String),
//     Other(String),
//     None,
// }

// トークンの種類を定義
#[derive(Debug, PartialEq)]
enum Token {
    IncludeLibrary(String),
    StorageClass(String),
    Type(String),
    Prototype(String),
    Struct(String),
    Other(String),
    None,
}

#[derive(Debug, PartialEq)]
pub enum StorageClassMember {
    Auto,
    Static,
    Extern,
    Register,
}

#[derive(Debug, PartialEq)]
pub enum TypeMember {
    Void,
    Bool,
    Char,
    Short,
    Int,
    Long,
    LLong,
    Float,
    Double,
}

fn parser_include_library(s: &str) -> IResult<&str, Token> {
    let (s, _) = tuple((multispace0, tag("#include"), multispace0))(s)?;
    // let code = s.clone();
    let (s, open) = nom::character::complete::one_of("<\"")(s)?;
    let (s, (library_name, _)) =
        delimited(multispace0, tuple((alpha1, tag(".h"))), multispace0)(s)?;
    let (s, _) = tag(match open {
        '<' => ">",
        '\"' => "\"",
        _ => panic!(),
    })(s)?;

    Ok((s, Token::IncludeLibrary(library_name.to_string())))
    // Ok((s, Token::IncludeLibrary(code.to_string())))
}

// プロトタイプ宣言を解析するパーサー
fn parse_storage_class(s: &str) -> IResult<&str, Token> {
    let (s, member) = opt(alt((
        tag("auto"),
        tag("static"),
        tag("extern"),
        tag("register"),
    )))(s)?;

    Ok((
        s,
        match member {
            Some("auto") => Token::StorageClass("auto".to_string()),
            Some("static") => Token::StorageClass("auto".to_string()),
            Some("extern") => Token::StorageClass("auto".to_string()),
            Some("register") => Token::StorageClass("auto".to_string()),
            _ => Token::None,
        },
    ))
}

fn parse_type(s: &str) -> IResult<&str, Token> {
    let (s, member) = tuple((
        opt(alt((tag("signed"), tag("unsigned")))),
        opt(alt((tag("short"), tag("long")))),
        opt(alt((
            tag("void"),
            tag("bool"),
            tag("char"),
            tag("int"),
            tag("long"),
            tag("float"),
            tag("double"),
        ))),
    ))(s)?;

    Ok((
        s,
        match member {
            (None, None, Some("void")) => Token::Type("void".to_string()),
            (None, None, Some("bool")) => Token::Type("bool".to_string()),
            (None, None, Some("char")) => Token::Type("char".to_string()),
            (Some("signed"), None, Some("char")) => Token::Type("signed char".to_string()),
            (Some("unsigned"), None, Some("char")) => Token::Type("unsigned char".to_string()),
            (None, Some("short"), None | Some("int")) => Token::Type("short".to_string()),
            (Some("signed"), Some("short"), None | Some("int")) => {
                Token::Type("signed short".to_string())
            }
            (Some("unsigned"), Some("short"), None | Some("int")) => {
                Token::Type("unsigned short".to_string())
            }
            (None, None, Some("int")) => Token::Type("int".to_string()),
            (Some("signed"), None, Some("int")) => Token::Type("signed int".to_string()),
            (Some("unsigned"), None, Some("int")) => Token::Type("unsigned int".to_string()),
            (None, Some("long"), None | Some("int")) => Token::Type("long".to_string()),
            (Some("signed"), Some("long"), None | Some("int")) => {
                Token::Type("signed long".to_string())
            }
            (Some("unsigned"), Some("long"), None | Some("int")) => {
                Token::Type("unsigned long".to_string())
            }
            (None, Some("long"), Some("long")) => Token::Type("long long".to_string()),
            (Some("signed"), Some("long"), Some("long")) => {
                Token::Type("signed long long".to_string())
            }
            (Some("unsigned"), Some("long"), Some("long")) => {
                Token::Type("unsigned long long".to_string())
            }
            _ => Token::None,
        },
    ))
}

fn parse_prototype(s: &str) -> IResult<&str, Token> {
    let (s, _) = multispace0(s)?;
    let code = s.clone();
    let (s, storage_class_token) = parse_storage_class(s)?;
    // dbg!(s, &storage_class_token);
    let (s, _) = multispace0(s)?;
    let (s, type_token) = parse_type(s)?;
    // dbg!(&type_token);
    let (s, _) = multispace0(s)?;
    let (s, funciton_name) = alpha1(s)?;
    // dbg!(&funciton_name);
    let (s, _) = multispace0(s)?;
    let (s, _) = delimited(tag("("), opt(alpha1), tag(")"))(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = tag(";")(s)?;

    Ok((s, Token::Prototype(code.to_string())))
}

// // 構造体宣言を解析するパーサー
// fn parse_struct(input: &str) -> IResult<&str, Token> {
//     let (input, _) = multispace0(input)?;
//     let (input, _) = tag("struct")(input)?;
//     let (input, _) = multispace0(input)?;
//     let (input, name) = alpha1(input)?;
//     let (input, _) = tag(";")(input)?;

//     Ok((input, Token::Struct(name.to_string())))
// }

// トークンを解析するパーサー
fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((
        parser_include_library,
        parse_prototype,
        // parse_struct,
        // map(rest_len, |_| Token::Other("".to_owned())),
    ))(input)
}

fn main() {
    // let input: &str = "int foo();\nstruct Bar { int x; };";
    let input: &str = "#include <dasda.h>\n#include \"stdio.h\"\nint foo();";
    let mut remaining = input;

    while !remaining.is_empty() {
        match parse_token(remaining) {
            Ok((rest, token)) => {
                remaining = rest;
                println!("{:?}", token);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
        dbg!(&remaining);
    }
}
