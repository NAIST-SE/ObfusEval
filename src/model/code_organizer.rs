use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;

use super::CodeOrganizer;
use crate::model::code_parser::{AnalyzedCodeData, CSourceCodeParser};
use crate::model::Parser;

pub struct TigressCodeOrganizer {}

impl CodeOrganizer for TigressCodeOrganizer {
    fn organize(src_path: &PathBuf, dst_path: &PathBuf) -> Result<()> {
        let source_code: String = fs::read_to_string(&src_path).unwrap();

        let analyze_result: Vec<AnalyzedCodeData> = CSourceCodeParser::parse(&source_code)?;
        // dbg!(&analyze_result);

        let extern_function_names: Vec<&str> = analyze_result
            .iter()
            .filter_map(|x| {
                if !x.is_extern() {
                    return None;
                }
                Some(x.identifier)
            })
            .map(|x| x.unwrap())
            .collect();

        let valid_extern_function_names: Vec<&str> = extern_function_names
            .iter()
            .filter_map(|&x| {
                if analyze_result
                    .iter()
                    .any(|data| data.is_given_function_included(x))
                {
                    return Some(x);
                }
                return None;
            })
            .collect();

        let mut organized_code: String = String::new();
        for x in analyze_result {
            // Exclude comment / muilti comment / space in global scope.
            if x.is_invalid_code_line() {
                continue;
            }

            // Exclude invalid extern function declaration (invalid: unused in function implementation)
            if x.is_extern() {
                if !valid_extern_function_names
                    .iter()
                    .any(|&valid_function_name| x.identifier == Some(valid_function_name))
                {
                    continue;
                }
            }

            // To avoid redefination，exclude timeval struct.
            if x.is_struct_type() {
                if x.identifier == Some("timeval") {
                    continue;
                }
            }

            // Exclude main and megaInit function
            if x.is_function_type() {
                if (x.identifier == Some("main")) || (x.identifier == Some("megaInit")) {
                    continue;
                }
            }

            // Exclude use of enum without previous declaration
            if x.is_enum_type() {
                // Comment: exclude instructions that only call enum (for now)
                if x.contents.len() == 1 && x.contents[0].ends_with(";") {
                    continue;
                }
            }

            organized_code += &x.contents.join("\n");
            organized_code += &"\n";
        }

        let mut include_library = vec![];
        if valid_extern_function_names
            .iter()
            .any(|x| x == &"printf" || x == &"sprintf")
        {
            include_library.push("#include <stdio.h>");
        }
        if valid_extern_function_names.iter().any(|x| x == &"strcat") {
            include_library.push("#include <string.h>");
        }
        if valid_extern_function_names
            .iter()
            .any(|x| x == &"rand" || x == &"*malloc")
        {
            include_library.push("#include <stdlib.h>");
        }
        organized_code = include_library.join("\n") + "\n\n" + &organized_code;

        let mut file: File = File::create(dst_path)?;
        write!(file, "{}", organized_code)?;

        Ok(())
    }
}
