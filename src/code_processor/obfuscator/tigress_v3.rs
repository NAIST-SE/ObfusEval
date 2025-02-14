use crate::code_processor::{
    parser::{c_language_parser::CLanguageParser, AnalyzedCodeData, Parser},
    Executor, ExecutorInfo, Organizer,
};
use std::io::Write;

use super::{Obfuscator, ObfuscatorSerializeModel, TransformationInfo};

#[derive(Debug)]
pub struct TigressV3 {
    pub executor: ExecutorInfo,
    pub common_parameter: Vec<String>,
    pub transformation_set: Vec<TransformationInfo>,
}

impl Executor for TigressV3 {
    fn label(&self) -> &str {
        &self.executor.label
    }
    fn name(&self) -> &str {
        &self.executor.name
    }
    fn version(&self) -> &str {
        &self.executor.version
    }
    fn bin_path(&self) -> &str {
        &self.executor.binary_path
    }
    fn bin_args(&self) -> Option<Vec<&str>> {
        match &self.executor.binary_args {
            Some(bin_args) => Some(bin_args.iter().map(String::as_str).collect()),
            _ => None,
        }
    }
}

impl Obfuscator for TigressV3 {
    fn new(executor: ExecutorInfo, value: ObfuscatorSerializeModel) -> Self {
        Self {
            executor: executor,
            common_parameter: value.common_parameter,
            transformation_set: value.transformation_set,
        }
    }
    fn has_post_process(&self) -> bool {
        true
    }

    fn transformation_set(&self) -> &Vec<TransformationInfo> {
        &self.transformation_set
    }

    fn args(
        &self,
        source: &[std::path::PathBuf],
        function_name: &str,
        transformation: &TransformationInfo,
        obfuscated_source_path: &std::path::PathBuf,
        temp_obfuscated_binary_path: &std::path::PathBuf,
    ) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        if let Some(bin_args) = self.bin_args() {
            bin_args.iter().for_each(|x| result.push(x.to_string()));
        }

        self.common_parameter
            .iter()
            .for_each(|x| result.push(x.to_string()));

        transformation
            .generate_parameter(function_name)
            .iter()
            .for_each(|x| {
                result.push(x.to_string());
            });

        source
            .iter()
            .for_each(|p| result.push(p.display().to_string()));

        result.push("-o".to_string());
        result.push(temp_obfuscated_binary_path.display().to_string());

        result.push(format!("--out={}", obfuscated_source_path.display()));

        result
    }

    fn pre_obfuscate(
        &self,
        working_directory: &std::path::PathBuf,
        source: &[std::path::PathBuf],
        function_name: &str,
        transformation: &TransformationInfo,
        temp_obfuscated_source_path: &std::path::PathBuf,
        dry_run: &bool,
    ) -> anyhow::Result<Vec<std::path::PathBuf>> {
        let (_, _, _, _, _) = (
            working_directory,
            function_name,
            transformation,
            temp_obfuscated_source_path,
            dry_run,
        );
        return Ok(vec![source[0].clone()]);
    }

    fn post_obfuscate(
        &self,
        working_directory: &std::path::PathBuf,
        function_name: &str,
        temp_obfuscated_source_path: &std::path::PathBuf,
        temp_obfuscated_binary_path: &std::path::PathBuf,
        obfuscated_source_path: &std::path::PathBuf,
        dry_run: &bool,
    ) -> anyhow::Result<()> {
        let (_, _) = (working_directory, function_name);

        if *dry_run {
            return Ok(());
        }

        if temp_obfuscated_binary_path.exists() {
            std::fs::remove_file(temp_obfuscated_binary_path)?;
        }

        TigressV3::organize(temp_obfuscated_source_path, obfuscated_source_path)?;

        Ok(())
    }
}

impl Organizer for TigressV3 {
    fn organize(
        src_path: &std::path::PathBuf,
        dst_path: &std::path::PathBuf,
    ) -> anyhow::Result<()> {
        let source_code = std::fs::read_to_string(&src_path)?;
        let analyze_result: Vec<AnalyzedCodeData> = CLanguageParser::parse(&source_code)?;

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

        let mut file = std::fs::File::create(dst_path)?;
        write!(file, "{}", organized_code)?;

        Ok(())
    }
}
