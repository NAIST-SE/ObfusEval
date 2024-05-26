use anyhow::{bail, Result};
use nom::error::ErrorKind;
use nom::multi::many0_count;
use nom::sequence::pair;
use std::{fs, path::PathBuf};

use super::c_token::Token;
use nom::character::complete::{alphanumeric1, one_of};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{alpha1, multispace0, multispace1, newline, not_line_ending},
    combinator::recognize,
    error::ParseError,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

use nom::character::complete::char;

pub trait Lexer {
    fn analyze(path: &PathBuf) -> Result<Vec<Token>>;
}

pub struct CLexer {}

impl Lexer for CLexer {
    fn analyze(input_file_path: &PathBuf) -> Result<Vec<Token>> {
        let mut tokens: Vec<Token> = vec![];

        let content: String = fs::read_to_string(input_file_path)?;
        let mut remaining: &str = &content;
        while !remaining.is_empty() {
            match CLexer::tokenize(&remaining) {
                Ok((rest, token)) => {
                    remaining = rest;
                    tokens.push(token);
                }
                Err(e) => {
                    dbg!(&remaining, &tokens);
                    bail!("Failed lexical analysis {:?}", e);
                }
            }
        }

        Ok(tokens)
    }
}

impl CLexer {
    fn tokenize(input: &str) -> IResult<&str, Token> {
        alt((
            CLexer::parse_keyword,
            CLexer::parse_punctuator_only_char,
            CLexer::parse_punctuator,
            CLexer::parse_pre_processor,
            CLexer::parse_identifier,
            CLexer::parse_string_literal_syntax,
            // キーワード関連のトークナイズ
            // CLexer::tokenize_allow,
            // CLexer::tokenize_parentheses_start,
            // CLexer::tokenize_parentheses_end,
            // CLexer::tokenize_curly_bracket_start,
            // CLexer::tokenize_curly_bracket_end,
            // CLexer::tokenize_simple_assginment_operator,
            // CLexer::tokenize_statement_end_syntax,
            // CLexer::tokenize_negation_evaluate_operator,
            // CLexer::tokenize_sequential_evaluate_operator,
            // #include 関連のトークナイズ
            CLexer::parse_include_library,
            CLexer::parse_include_local_library,
            // // 修飾子・型関連のトークナイズ
            // CLexer::tokenize_storage_class,
            // CLexer::tokenize_struct,
            // CLexer::tokenize_type,
            // // 変数・関数関連のトークナイズ
            // CLexer::tokenize_double_quotation,
            // CLexer::tokenize_variable,
            // CLexer::tokenize_varadic_arguments,
            // コメント関連のトークナイズ
            CLexer::parse_single_line_comment,
            CLexer::parse_multi_line_comment,
        ))(input)
    }
}

// 修飾子・型関連のトークナイズ
impl CLexer {
    // fn tokenize_storage_class(s: &str) -> IResult<&str, Token> {
    //     let (s, _) = multispace0(s)?;
    //     let (s, member) = opt(alt((
    //         tag("auto"),
    //         tag("register"),
    //         tag("static"),
    //         tag("extern"),
    //         tag("typedef"),
    //     )))(s)?;

    //     if let Some(member) = member {
    //         Ok((
    //             s,
    //             match member {
    //                 "auto" => Token::StorageClass(StorageClassMember::Auto),
    //                 "register" => Token::StorageClass(StorageClassMember::Register),
    //                 "static" => Token::StorageClass(StorageClassMember::Static),
    //                 "extern" => Token::StorageClass(StorageClassMember::Extern),
    //                 "typedef" => Token::StorageClass(StorageClassMember::TypeDef),
    //                 _ => panic!(),
    //             },
    //         ))
    //     } else {
    //         return fail(s);
    //     }
    // }

    // fn tokenize_struct(s: &str) -> IResult<&str, Token> {
    //     let (s, _) = tuple((multispace0, tag("struct"), multispace0))(s)?;
    //     let (s, struct_name) = delimited(
    //         multispace0,
    //         recognize(pair(
    //             alt((alpha1, tag("_"))),
    //             many0_count(alt((alphanumeric1, tag("_")))),
    //         )),
    //         multispace0,
    //     )(s)?;
    //     Ok((s, Token::Struct(struct_name.to_string())))
    // }

    // fn tokenize_type(s: &str) -> IResult<&str, Token> {
    //     let (s, _) = multispace0(s)?;
    //     let (s, member) = tuple((
    //         opt(alt((tag("signed"), tag("unsigned")))),
    //         opt(alt((tag("short"), tag("long")))),
    //         opt(alt((
    //             tag("void"),
    //             tag("bool"),
    //             tag("char"),
    //             tag("int"),
    //             tag("long"),
    //             tag("float"),
    //             tag("double"),
    //         ))),
    //     ))(s)?;

    //     if member.0.is_none() && member.1.is_none() && member.2.is_none() {
    //         return fail(s);
    //     }

    //     Ok((
    //         s,
    //         match member {
    //             (None, None, Some("void")) => Token::Type(TypeMember::Void),
    //             (None, None, Some("bool")) => Token::Type(TypeMember::Bool),
    //             (None, None, Some("char")) => Token::Type(TypeMember::Char),
    //             (Some("signed"), None, Some("char")) => Token::Type(TypeMember::SignedChar),
    //             (Some("unsigned"), None, Some("char")) => Token::Type(TypeMember::UnsignedChar),
    //             (None, Some("short"), None | Some("int")) => Token::Type(TypeMember::Short),
    //             (Some("signed"), Some("short"), None | Some("int")) => {
    //                 Token::Type(TypeMember::SignedShort)
    //             }
    //             (Some("unsigned"), Some("short"), None | Some("int")) => {
    //                 Token::Type(TypeMember::UnsignedShort)
    //             }
    //             (None, None, Some("int")) => Token::Type(TypeMember::Int),
    //             (Some("signed"), None, Some("int")) => Token::Type(TypeMember::SignedInt),
    //             (Some("unsigned"), None, Some("int")) => Token::Type(TypeMember::UnsignedInt),
    //             (None, Some("long"), None | Some("int")) => Token::Type(TypeMember::Long),
    //             (Some("signed"), Some("long"), None | Some("int")) => {
    //                 Token::Type(TypeMember::SignedLong)
    //             }
    //             (Some("unsigned"), Some("long"), None | Some("int")) => {
    //                 Token::Type(TypeMember::UnsignedLong)
    //             }
    //             (None, Some("long"), Some("long")) => Token::Type(TypeMember::LongLong),
    //             (Some("signed"), Some("long"), Some("long")) => {
    //                 Token::Type(TypeMember::SingedLongLong)
    //             }
    //             (Some("unsigned"), Some("long"), Some("long")) => {
    //                 Token::Type(TypeMember::UnsingedLongLong)
    //             }
    //             (None, None, Some("float")) => Token::Type(TypeMember::Float),
    //             (None, None, Some("double")) => Token::Type(TypeMember::Double),
    //             _ => Token::None,
    //         },
    //     ))
    // }
}

// 変数・関数関連のトークナイズ
impl CLexer {
    // fn tokenize_double_quotation(s: &str) -> IResult<&str, Token> {
    //     let (s, _) = multispace0(s)?;
    //     let (s, string_value) = delimited(tag("\""), take_until("\""), tag("\""))(s)?;
    //     Ok((s, Token::DoubleQuotation(string_value.to_string())))
    // }

    // fn tokenize_variable(s: &str) -> IResult<&str, Token> {
    //     let (s, variable_name) = delimited(
    //         multispace0,
    //         // take_till(|c| c == '\"' || c == ';'),
    //         recognize(pair(
    //             alt((alphanumeric1, tag("_"), tag("*"))),
    //             many0_count(alt((alphanumeric1, tag("_"), tag("*")))),
    //         )),
    //         multispace0,
    //     )(s)?;
    //     Ok((s, Token::Variable(variable_name.to_string())))
    // }
}

// キーワード関連のトークナイズ
impl CLexer {
    fn parse_keyword(s: &str) -> IResult<&str, Token> {
        let (s, word) = delimited(
            multispace0,
            take_till(|c| c == '\"' || c == ';' || c == ' '),
            multispace1,
        )(s)?;

        if let Some(keyword) = Token::get_keywords().into_iter().find(|x| x == &word) {
            Ok((s, Token::Keyword(keyword.to_string())))
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                s,
                nom::error::ErrorKind::TakeTill1,
            )))
        }
    }

    fn parse_identifier(s: &str) -> IResult<&str, Token> {
        let (s, identifier) = delimited(
            multispace0,
            recognize(pair(
                alt((alphanumeric1, tag("_"))),
                many0_count(alt((alphanumeric1, tag("_")))),
            )),
            multispace0,
        )(s)?;

        if let Ok(digit) = identifier.parse::<i32>() {
            return Ok((s, Token::IdentifierDigit(digit)));
        }
        if let Some(first_char) = identifier.chars().nth(0) {
            if first_char.is_alphabetic() || first_char == '_' {
                return Ok((s, Token::Identifier(identifier.to_string())));
            }
        }
        Err(nom::Err::Error(nom::error::Error::new(
            s,
            nom::error::ErrorKind::TakeTill1,
        )))
    }

    fn parse_punctuator_only_char(s: &str) -> IResult<&str, Token> {
        let (s, _) = multispace0(s)?;
        let (s, punctuator) = one_of(Token::get_punctuators_as_str_only_char())(s)?;
        Ok((s, Token::Punctuator(punctuator.to_string())))
    }

    fn parse_punctuator(s: &str) -> IResult<&str, Token> {
        let (s, punctuator) = delimited(
            multispace0,
            tag(Token::get_puctuator_for_varadic_arguments()),
            multispace0,
        )(s)?;
        Ok((s, Token::Punctuator(punctuator.to_string())))
    }

    fn parse_pre_processor(s: &str) -> IResult<&str, Token> {
        let (s, word) = delimited(
            multispace0,
            take_till(|c| c == '\"' || c == ';' || c == ' '),
            multispace1,
        )(s)?;

        if let Some(pre_processor) = Token::get_pre_processors().into_iter().find(|x| x == &word) {
            Ok((s, Token::PreProcessor(pre_processor.to_string())))
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                s,
                nom::error::ErrorKind::TakeTill1,
            )))
        }
    }

    fn parse_string_literal_syntax(s: &str) -> IResult<&str, Token> {
        let (s, (_, literal, _)) = tuple((
            multispace0,
            delimited(char('\"'), take_until("\""), char('\"')),
            multispace0,
        ))(s)?;
        Ok((s, Token::StringLiteralSyntax(literal.to_string())))
    }

    // // ここから複数文字文字キーワード
    // fn tokenize_keywsord<'a>(
    //     s: &'a str,
    //     token_class: Token,
    //     key: &'a str,
    // ) -> IResult<&'a str, Token> {
    //     let (s, _) = delimited(multispace0, tag(key), multispace0)(s)?;
    //     Ok((s, token_class))
    // }

    // fn tokenize_varadic_arguments(s: &str) -> IResult<&str, Token> {
    //     CLexer::tokenize_keyword(s, Token::VaradicArguments, "...")
    // }

    // fn tokenize_allow(s: &str) -> IResult<&str, Token> {
    //     CLexer::tokenize_keyword(s, Token::Allow, "->")
    // }

    // // ここから1文字キーワード
    // fn tokenize_single_keyword(s: &str, token_class: Token, c: char) -> IResult<&str, Token> {
    //     let (s, _) = delimited(multispace0, char(c), multispace0)(s)?;
    //     Ok((s, token_class))
    // }

    // fn tokenize_parentheses_start(s: &str) -> IResult<&str, Token> {
    //     CLexer::tokenize_single_keyword(s, Token::ParenthesesStart, '(')
    // }

    // fn tokenize_parentheses_end(s: &str) -> IResult<&str, Token> {
    //     CLexer::tokenize_single_keyword(s, Token::ParenthesesEnd, ')')
    // }

    // fn tokenize_curly_bracket_start(s: &str) -> IResult<&str, Token> {
    //     CLexer::tokenize_single_keyword(s, Token::CurlyBracketStart, '{')
    // }

    // fn tokenize_curly_bracket_end(s: &str) -> IResult<&str, Token> {
    //     CLexer::tokenize_single_keyword(s, Token::CurlyBracketEnd, '}')
    // }

    // fn tokenize_plus(s: &str) -> IResult<&str, Token> {
    //     CLexer::tokenize_single_keyword(s, Token::Plus, '+')
    // }

    // fn tokenize_minus(s: &str) -> IResult<&str, Token> {
    //     CLexer::tokenize_single_keyword(s, Token::Minus, '-')
    // }

    // fn tokenize_statement_end_syntax(s: &str) -> IResult<&str, Token> {
    //     CLexer::tokenize_single_keyword(s, Token::StatementEndStyntax, ';')
    // }

    // fn tokenize_negation_evaluate_operator(s: &str) -> IResult<&str, Token> {
    //     CLexer::tokenize_single_keyword(s, Token::NegationEvaluatingOperator, '!')
    // }

    // fn tokenize_simple_assginment_operator(s: &str) -> IResult<&str, Token> {
    //     CLexer::tokenize_single_keyword(s, Token::SimpleAssignmentOperator, '=')
    // }

    // fn tokenize_sequential_evaluate_operator(s: &str) -> IResult<&str, Token> {
    //     CLexer::tokenize_single_keyword(s, Token::SequentialEvaluatingOperator, ',')
    // }
}

impl CLexer {
    fn parse_include_library(s: &str) -> IResult<&str, Token> {
        let (s, library_name) = CLexer::parse_include_library_common(s, '\"', '\"')?;
        Ok((s, Token::IncludeLibrarySyntax(library_name.to_string())))
    }

    fn parse_include_local_library(s: &str) -> IResult<&str, Token> {
        let (s, library_name) = CLexer::parse_include_library_common(s, '<', '>')?;
        Ok((
            s,
            Token::IncludeLibraryLocalSyntax(library_name.to_string()),
        ))
    }

    fn parse_include_library_common(s: &str, open: char, close: char) -> IResult<&str, &str> {
        let (s, _) = tuple((multispace0, tag("#include"), multispace1))(s)?;
        let (s, (_, library_name, _)) = tuple((
            multispace0,
            delimited(char(open), take_until(&close.to_string()[..]), char(close)),
            multispace0,
        ))(s)?;
        Ok((s, library_name))
    }

    fn parse_single_line_comment(s: &str) -> IResult<&str, Token> {
        let (s, _) = multispace0(s)?;
        let (s, comment) = delimited(tag("//"), not_line_ending, newline)(s)?;
        Ok((s, Token::SingleLineCommentSyntax(comment.to_string())))
    }

    fn parse_multi_line_comment(s: &str) -> IResult<&str, Token> {
        let (s, _) = multispace0(s)?;
        let (s, comment) = delimited(tag("/*"), take_until("*/"), tag("*/"))(s)?;
        Ok((s, Token::MultiLineCommentSyntax(comment.to_string())))
    }
}
