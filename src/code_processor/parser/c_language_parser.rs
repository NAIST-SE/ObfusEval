use std::marker::PhantomData;

use super::{AnalyzedCodeData, InvalidContentTypeKinds, Parser, StorageClassKinds, TypeKinds};

pub struct CLanguageParser<'a>(PhantomData<&'a ()>);

impl<'a> Parser<'a> for CLanguageParser<'a> {
    fn parse(source_code: &'a String) -> anyhow::Result<Vec<AnalyzedCodeData<'a>>> {
        let mut result: Vec<AnalyzedCodeData> = vec![];

        let mut scope_num: isize = 0;
        let mut bracket_scope_num: isize = 0;
        let mut paren_scope_num: isize = 0;

        let mut is_current_multi_comment: bool = false;
        let mut is_current_implementation: bool = false;
        let mut current_storage_class_kind: StorageClassKinds = StorageClassKinds::Auto;
        let mut current_type_kind: Option<TypeKinds> = None;
        let mut current_identifier: &str = "";
        let mut current_contents: Vec<&str> = vec![];
        for line in source_code.lines() {
            if scope_num == 0 {
                // コメント，空白行の処理
                if let Some(comment_type) = Self::parse_comment(line) {
                    match comment_type {
                        InvalidContentTypeKinds::Comment => {
                            result
                                .push(AnalyzedCodeData::create_from_invalid_code_line(vec![line]));
                            continue;
                        }
                        InvalidContentTypeKinds::MultiComment => {
                            is_current_multi_comment = true;
                        }
                        InvalidContentTypeKinds::Blank => {
                            result.push(AnalyzedCodeData::create_from_invalid_code_line(vec![]));
                            continue;
                        }
                    }
                } else {
                    current_storage_class_kind = Self::parse_storage_class(line);
                    if !is_current_implementation {
                        current_type_kind = Self::parse_type_name(line);
                        current_identifier = match current_type_kind {
                            Some(TypeKinds::Enum) => Self::parse_enum_name(line),
                            Some(TypeKinds::Function(_)) => Self::parse_function_name(line),
                            Some(TypeKinds::Struct) => Self::parse_struct_name(line),
                            Some(TypeKinds::TypeDef) => Self::parse_typedef_name(line),
                            Some(TypeKinds::Union) => Self::parse_union_name(line),
                            None => current_identifier,
                        };
                    } else {
                        current_type_kind = Some(TypeKinds::Function("Implementation"));
                        current_identifier = current_identifier;
                    }
                }
            }
            current_contents.push(line);

            // 複数行のコメントにおける処理
            if is_current_multi_comment {
                if line.ends_with("*/") {
                    result.push(AnalyzedCodeData::create_from_invalid_code_line(
                        current_contents.clone(),
                    ));
                    current_contents.clear();
                    is_current_multi_comment = false;
                }
                continue;
            }

            // スコープの計算
            bracket_scope_num += line.matches("{").collect::<Vec<&str>>().len() as isize
                - line.matches("}").collect::<Vec<&str>>().len() as isize;
            paren_scope_num += line.matches("(").collect::<Vec<&str>>().len() as isize
                - line.matches(")").collect::<Vec<&str>>().len() as isize;
            scope_num = bracket_scope_num + paren_scope_num;

            if scope_num == 0 {
                if line.ends_with(";") {
                    ()
                } else if is_current_implementation && line.ends_with("}") {
                    is_current_implementation = false;
                } else if let Some(TypeKinds::Function(_)) = current_type_kind {
                    is_current_implementation = true;
                    continue;
                } else {
                    continue;
                }

                result.push(AnalyzedCodeData::new(
                    current_storage_class_kind.clone(),
                    current_type_kind.clone(),
                    current_identifier,
                    current_contents.clone(),
                ));
                current_contents.clear();
            }
        }

        Ok(result)
    }
}

impl<'a> CLanguageParser<'a> {
    fn parse_comment(line: &str) -> Option<InvalidContentTypeKinds> {
        let Some(type_name) = line.split_whitespace().nth(0) else {
            return Some(InvalidContentTypeKinds::Blank);
        };

        match type_name {
            "//" => Some(InvalidContentTypeKinds::Comment),
            "/*" => Some(InvalidContentTypeKinds::MultiComment),
            _ => None,
        }
    }

    fn parse_storage_class(line: &str) -> StorageClassKinds {
        match line.split_whitespace().nth(0).unwrap() {
            "extern" => StorageClassKinds::External,
            _ => StorageClassKinds::Auto,
        }
    }

    fn parse_type_name(line: &str) -> Option<TypeKinds> {
        match line.split_whitespace().nth(0).unwrap() {
            "typedef" => Some(TypeKinds::TypeDef),
            "struct" => Some(TypeKinds::Struct),
            "enum" => Some(TypeKinds::Enum),
            "union" => Some(TypeKinds::Union),
            _ => {
                let Some(type_identifier_start) = line.find(" ") else {
                    return None;
                };
                let Some(function_name_end) = line.find("(") else {
                    return None;
                };
                let Some(type_identifier_end) = line[..function_name_end].rfind(" ") else {
                    return None;
                };
                let identifier = if type_identifier_end > type_identifier_start + 1 {
                    line[type_identifier_start + 1..type_identifier_end].trim()
                } else {
                    line[..type_identifier_end].trim()
                };
                Some(TypeKinds::Function(identifier))
            }
        }
    }

    fn parse_enum_name(line: &str) -> &str {
        line.split_whitespace().nth(1).unwrap()
    }

    fn parse_function_name(line: &str) -> &str {
        let mut end: usize = line.rfind("(").unwrap();
        loop {
            let Some(t) = line.chars().nth(end - 1) else {
                break;
            };
            if t != ')' {
                break;
            }
            end -= 1;
        }
        let start: usize = line[..end].rfind(" ").unwrap();
        line[start + 1..end].trim()
    }

    fn parse_struct_name(line: &str) -> &str {
        line.split_whitespace().nth(1).unwrap()
    }

    fn parse_typedef_name(line: &str) -> &str {
        let end: usize = if let Some(e1) = line.find(";") {
            e1
        } else if let Some(e2) = line.find("{") {
            e2
        } else {
            return "";
        };
        let start: usize = line[..end].rfind(" ").unwrap();
        line[start + 1..end].trim()
    }

    fn parse_union_name(line: &str) -> &str {
        line.split_whitespace().nth(1).unwrap()
    }
}
