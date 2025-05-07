use crate::code_processor::{Executor, ExecutorInfo};

use super::{Obfuscator, ObfuscatorOption, ObfuscatorSerializeModel, TransformationInfo};

#[derive(Debug)]
pub struct TigressV4 {
    pub executor: ExecutorInfo,
    pub common_parameter: Vec<String>,
    pub transformation_set: Vec<TransformationInfo>,
}

impl Executor for TigressV4 {
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

impl Obfuscator for TigressV4 {
    fn new(executor: ExecutorInfo, value: ObfuscatorSerializeModel) -> Self {
        Self {
            executor: executor,
            common_parameter: value.common_parameter,
            transformation_set: value.transformation_set,
        }
    }

    fn transformation_set(&self) -> &Vec<TransformationInfo> {
        &self.transformation_set
    }

    fn args(
        &self,
        input: &[std::path::PathBuf],
        file_name: &str,
        option: &ObfuscatorOption,
    ) -> Vec<String> {
        let target = option.raw_obfuscated_file(file_name);
        let temp_elf = option.temp_elf(file_name);

        [
            vec![self.bin_path().to_string()],
            self.common_parameter.clone(),
            option
                .transformation
                .generate_parameter(&option.target_function),
            input
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<String>>(),
            vec![
                format!("-o {}", temp_elf.display()),
                format!("--out={}", target.display()),
            ],
        ]
        .concat()
    }

    fn post_obfuscate(
        &self,
        source: &std::path::PathBuf,
        target: &std::path::PathBuf,
        file_name: &str,
        option: &ObfuscatorOption,
        dry_run: &bool,
    ) -> anyhow::Result<()> {
        if *dry_run {
            return Ok(());
        }

        let temp_elf = option.temp_elf(file_name);
        if temp_elf.exists() {
            std::fs::remove_file(temp_elf)?;
        }

        let (_, _) = (source, target);
        // self.organize(source, target)?;

        Ok(())
    }
}

// impl Organizer for TigressV4 {
//     fn organize(
//         src_path: &std::path::PathBuf,
//         dst_path: &std::path::PathBuf,
//     ) -> anyhow::Result<()> {
//         let source_code = std::fs::read_to_string(&src_path)?;
//         let analyze_result: Vec<AnalyzedCodeData> = CLanguageParser::parse(&source_code)?;

//         let extern_function_names: Vec<&str> = analyze_result
//             .iter()
//             .filter_map(|x| {
//                 if !x.is_extern() {
//                     return None;
//                 }
//                 Some(x.identifier)
//             })
//             .map(|x| x.unwrap())
//             .collect();

//         let valid_extern_function_names: Vec<&str> = extern_function_names
//             .iter()
//             .filter_map(|&x| {
//                 if analyze_result
//                     .iter()
//                     .any(|data| data.is_given_function_included(x))
//                 {
//                     return Some(x);
//                 }
//                 return None;
//             })
//             .collect();

//         let mut organized_code: String = String::new();
//         for x in analyze_result {
//             // Exclude comment / muilti comment / space in global scope.
//             if x.is_invalid_code_line() {
//                 continue;
//             }

//             // Exclude invalid extern function declaration (invalid: unused in function implementation)
//             if x.is_extern() {
//                 if !valid_extern_function_names
//                     .iter()
//                     .any(|&valid_function_name| x.identifier == Some(valid_function_name))
//                 {
//                     continue;
//                 }
//             }

//             // To avoid redefination，exclude timeval struct.
//             if x.is_struct_type() {
//                 if x.identifier == Some("timeval") {
//                     continue;
//                 }
//             }

//             // Exclude main and megaInit function
//             if x.is_function_type() {
//                 if (x.identifier == Some("main")) || (x.identifier == Some("megaInit")) {
//                     continue;
//                 }
//             }

//             // Exclude use of enum without previous declaration
//             if x.is_enum_type() {
//                 // Comment: exclude instructions that only call enum (for now)
//                 if x.contents.len() == 1 && x.contents[0].ends_with(";") {
//                     continue;
//                 }
//             }

//             organized_code += &x.contents.join("\n");
//             organized_code += &"\n";
//         }

//         let mut include_library = vec![];
//         if valid_extern_function_names
//             .iter()
//             .any(|x| x == &"printf" || x == &"sprintf")
//         {
//             include_library.push("#include <stdio.h>");
//         }
//         if valid_extern_function_names.iter().any(|x| x == &"strcat") {
//             include_library.push("#include <string.h>");
//         }
//         if valid_extern_function_names
//             .iter()
//             .any(|x| x == &"rand" || x == &"*malloc")
//         {
//             include_library.push("#include <stdlib.h>");
//         }
//         organized_code = include_library.join("\n") + "\n\n" + &organized_code;

//         let mut file = std::fs::File::create(dst_path)?;
//         write!(file, "{}", organized_code)?;

//         Ok(())
//     }
// }
