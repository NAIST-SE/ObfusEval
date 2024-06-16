use std::collections::HashSet;
use std::io::Write;
use std::{
    fs::{self, File},
    path::PathBuf,
};

use anyhow::{bail, Result};

use super::CodeOrganizer;

pub struct TigressCodeOrganizer {}

impl CodeOrganizer for TigressCodeOrganizer {
    fn organize(src_path: &PathBuf, dst_path: &PathBuf) -> Result<()> {
        let source_code: String = fs::read_to_string(&src_path).unwrap();

        let invalid_extern_function_names: HashSet<&str> = Self::analyze(&source_code)?;

        let adjusted_source_code: String =
            Self::organize_core(&source_code, invalid_extern_function_names);

        let mut file: File = File::create(dst_path)?;
        write!(file, "{}", adjusted_source_code)?;

        Ok(())
    }
}

impl TigressCodeOrganizer {
    // todo: 1行ずつイテレータを回す．スコープごとに区切る．イテレータの最後に登録されている関数群を実行する．関数群は現在のスコープ情報を受け取る．
    fn analyze(source_code: &String) -> Result<HashSet<&str>> {
        let mut extern_function_names: HashSet<&str> = HashSet::new();
        let mut implemented_function_names: HashSet<&str> = HashSet::new();
        let mut used_function_names: HashSet<&str> = HashSet::new();

        let mut preline: &str = "";
        let mut scope_num: isize = -1;
        for line in source_code.lines() {
            if line.starts_with("extern") && scope_num < 0 {
                if let Ok(functoin_name) = Self::parse_function_name(line) {
                    extern_function_names.insert(functoin_name);
                }
            }

            if line.contains("{") {
                scope_num += 1;

                // 関数のスコープに入った直前の行から関数名を取得する
                if scope_num == 0 {
                    if let Ok(functoin_name) = Self::parse_function_name(preline) {
                        implemented_function_names.insert(functoin_name);
                    }
                }
            }
            if scope_num >= 0 {
                // 外部関数名が行に含まれている場合は，その外部関数名を used_function_names に登録する．
                if let Some(used_function_name) = extern_function_names
                    .iter()
                    .find(|function_name| line.contains(*function_name))
                {
                    used_function_names.insert(used_function_name);
                }
            }
            if line.contains("}") {
                scope_num -= 1;
                if scope_num == 0 {
                    scope_num = -1;
                }
            }

            // グローバルスコープなら直前の行を保持しておく
            if scope_num < 0 {
                preline = line;
            }
        }

        dbg!(&extern_function_names);
        dbg!(&implemented_function_names);
        dbg!(&used_function_names);
        dbg!(&extern_function_names.difference(&used_function_names));
        dbg!("");

        // invalid_extern_function_names
        Ok(used_function_names)
    }

    fn parse_function_name(line: &str) -> Result<&str> {
        // dbg!(&line);
        if let Some(end) = line.find("(") {
            // dbg!(&line);
            if let Some(start) = line[..end].rfind(" ") {
                // dbg!(&line);
                let function_name = line[start + 1..end].trim();
                return if function_name.starts_with("*") {
                    Ok(&function_name[1..])
                } else {
                    Ok(function_name)
                };
            }
        }

        bail!("Failed to get function names: \"(\" is nof found.");
    }

    fn organize_core(source_code: &String, analyze_result: HashSet<&str>) -> String {
        String::new()
    }
}

// impl CodeAdjuster for TigressCodeAdjuster {
//     fn adjust_code(&self) -> Result<String> {
//         // もっと丁寧にやるべきだが，とりあえずTigressでは動くためOKとする
//         // Todo: 定義前enumの削除
//         // // Todo: 定義されているenumを検出．プロトタイプ宣言しているところを削除
//         let result = self
//             .remove_extern_function()
//             .remove_function("main")
//             .remove_function("megaInit")
//             .remove_struct_timeval()
//             .remove_enum_declaration()
//             .insert_cstdlib()
//             .insert_cstdio();

//         result
//             .adjusted_source_code
//             .with_context(|| format!("Adjust Error"))
//     }
// }

// impl TigressCodeAdjuster {
//     fn get_target_code(&self) -> Lines<'_> {
//         match &self.adjusted_source_code {
//             Some(target) => target.lines(),
//             None => self.source_code.lines(),
//         }
//     }

//     fn set_adjusted_code(&self, adjusted_code: String) -> Self {
//         TigressCodeAdjuster {
//             file_path: self.file_path.clone(),
//             source_code: self.source_code.clone(),
//             adjusted_source_code: Some(adjusted_code),
//         }
//     }

//     fn remove_extern_function(&self) -> Self {
//         let mut adjusted_code = String::new();

//         let mut scope_num: isize = -1;
//         let mut function_code = String::new();
//         for line in self.get_target_code() {
//             if line.trim().starts_with("extern") {
//                 scope_num = 0;
//             }

//             if scope_num < 0 {
//                 adjusted_code.push_str(line);
//                 adjusted_code.push('\n');
//                 continue;
//             }

//             if line.contains(") ;") {
//                 scope_num -= 1;
//             }

//             function_code.push_str(line);
//             function_code.push('\n');
//         }

//         // dbg!(function_code);
//         self.set_adjusted_code(adjusted_code)
//     }

//     fn insert_cstdlib(&self) -> Self {
//         let mut adjusted_code = String::new();

//         let mut is_cstdlib_included = false;
//         for line in self.get_target_code() {
//             if line.contains("malloc") || line.contains("rand") {
//                 if is_cstdlib_included {
//                     continue;
//                 }
//                 adjusted_code.insert_str(0, "#include <cstdlib>\n");
//                 is_cstdlib_included = true;
//             }
//             adjusted_code.push_str(line);
//             adjusted_code.push_str("\n");
//         }

//         self.set_adjusted_code(adjusted_code)
//     }

//     fn insert_cstdio(&self) -> Self {
//         let mut adjusted_code = String::new();

//         let mut is_cstdio_included = false;
//         for line in self.get_target_code() {
//             if line.contains("printf") {
//                 if is_cstdio_included {
//                     continue;
//                 }
//                 adjusted_code.insert_str(0, "#include <cstdio>\n");
//                 is_cstdio_included = true;
//             }
//             adjusted_code.push_str(line);
//             adjusted_code.push_str("\n");
//         }

//         self.set_adjusted_code(adjusted_code)
//     }

//     fn remove_function(&self, name: &str) -> Self {
//         let mut adjusted_code = String::new();

//         let function_name = format!("{}(", name);
//         let mut scope_num: isize = -1;
//         let mut function_code = String::new();
//         for line in self.get_target_code() {
//             if scope_num < 0 && line.contains(&function_name) {
//                 scope_num = 0;
//             }

//             if scope_num < 0 {
//                 adjusted_code.push_str(line);
//                 adjusted_code.push('\n');
//                 continue;
//             }

//             if line.contains("{") {
//                 scope_num += 1;
//             }
//             if line.contains("}") {
//                 scope_num -= 1;
//                 if scope_num == 0 {
//                     scope_num = -1;
//                 }
//             }

//             if line.contains(") ;") {
//                 scope_num = -1;
//             }
//             // dbg!(format!("{} {}", &scope_num, &line));

//             function_code.push_str(line);
//             function_code.push('\n');
//         }

//         self.set_adjusted_code(adjusted_code)
//     }

//     fn remove_struct_timeval(&self) -> Self {
//         let mut adjusted_code = String::new();

//         let mut scope_num: isize = -1;
//         let mut function_code = String::new();
//         for line in self.get_target_code() {
//             if line.contains(&"struct timeval {") {
//                 scope_num = 0;
//             }

//             if scope_num < 0 {
//                 adjusted_code.push_str(line);
//                 adjusted_code.push('\n');
//                 continue;
//             }

//             if line.contains("};") {
//                 scope_num -= 1;
//             }

//             function_code.push_str(line);
//             function_code.push('\n');
//         }

//         // dbg!(function_code);
//         self.set_adjusted_code(adjusted_code)
//     }

//     fn remove_enum_declaration(&self) -> Self {
//         let mut adjusted_code = String::new();

//         for line in self.get_target_code() {
//             if line.trim().starts_with("enum") && line.ends_with(";") {
//                 continue;
//             }

//             adjusted_code.push_str(line);
//             adjusted_code.push('\n');
//             continue;
//         }

//         // dbg!(function_code);
//         self.set_adjusted_code(adjusted_code)
//     }
// }
