use anyhow::Result;
use clap::Parser;
use std::fs;
use std::io::Write;
use std::{fs::File, path::PathBuf};

use crate::model::{code::CodeInfo, dataset::Dataset};

use super::Command;

#[derive(Parser)]
pub struct AdjustCodeCommand {
    #[arg(help = "Path to the json file that manages dataset repository")]
    pub dataset_json_path: PathBuf,

    #[arg(long = "target", help = "Target source code name")]
    pub target: Option<String>,
}

pub struct AdjustCodeCommandCore {
    dataset: Dataset,
    target: Option<String>,
}

impl AdjustCodeCommandCore {
    pub fn new(args: AdjustCodeCommand) -> Self {
        Self {
            dataset: Dataset::new(&args.dataset_json_path),
            target: args.target,
        }
    }
}

impl AdjustCodeCommandCore {
    fn insert_cstdlib(contents: &str) -> String {
        let mut is_cstdlib_included = false;
        let mut modified_contents = String::new();

        for line in contents.lines() {
            if line.contains("malloc") || line.contains("rand") {
                if is_cstdlib_included {
                    continue;
                }
                modified_contents.insert_str(0, "#include <cstdlib>\n");
                is_cstdlib_included = true;
            }
            modified_contents.push_str(line);
            modified_contents.push_str("\n");
        }

        modified_contents
    }

    fn remove_extern_function(contents: &str) -> String {
        let mut scope_num: isize = -1;
        let mut modified_contents = String::new();
        let mut function_code = String::new();

        for line in contents.lines() {
            if line.trim().starts_with("extern") {
                scope_num = 0;
            }

            if scope_num < 0 {
                modified_contents.push_str(line);
                modified_contents.push('\n');
                continue;
            }

            if line.contains(") ;") {
                scope_num -= 1;
            }

            function_code.push_str(line);
            function_code.push('\n');
        }

        // dbg!(function_code);
        modified_contents
    }

    fn remove_function(contents: &str, name: &str) -> String {
        let function_name = format!("{}(", name);
        let mut scope_num: isize = -1;
        let mut modified_contents = String::new();
        let mut function_code = String::new();

        for line in contents.lines() {
            if scope_num < 0 && line.contains(&function_name) {
                scope_num = 0;
            }

            if scope_num < 0 {
                modified_contents.push_str(line);
                modified_contents.push('\n');
                continue;
            }

            if line.contains("{") {
                scope_num += 1;
            }
            if line.contains("}") {
                scope_num -= 1;
                if scope_num == 0 {
                    scope_num = -1;
                }
            }

            if line.contains(") ;") {
                scope_num = -1;
            }
            // dbg!(format!("{} {}", &scope_num, &line));

            function_code.push_str(line);
            function_code.push('\n');
        }

        // dbg!(function_code);
        modified_contents
    }

    fn remove_struct_timeval(contents: &str) -> String {
        let mut scope_num: isize = -1;
        let mut modified_contents = String::new();
        let mut function_code = String::new();

        for line in contents.lines() {
            if line.contains(&"struct timeval {") {
                scope_num = 0;
            }

            if scope_num < 0 {
                modified_contents.push_str(line);
                modified_contents.push('\n');
                continue;
            }

            if line.contains("};") {
                scope_num -= 1;
            }

            function_code.push_str(line);
            function_code.push('\n');
        }

        // dbg!(function_code);
        modified_contents
    }

    fn remove_enum_declaration(contents: &str) -> String {
        let mut modified_contents = String::new();

        for line in contents.lines() {
            if line.trim().starts_with("enum") && line.ends_with(";") {
                continue;
            }

            modified_contents.push_str(line);
            modified_contents.push('\n');
            continue;
        }

        // dbg!(function_code);
        modified_contents
    }

    fn main(contents: String) -> String {
        let mut modified_contents: String;

        // もっと丁寧にやるべきだが，とりあえずTigressでは動くためOKとする
        // Todo: LINQスタイルな関数に調整する
        // Todo: 構文解析などを用いた処理に変更する
        modified_contents = AdjustCodeCommandCore::insert_cstdlib(&contents);
        modified_contents = AdjustCodeCommandCore::remove_extern_function(&modified_contents);
        modified_contents = AdjustCodeCommandCore::remove_function(&modified_contents, "main");
        modified_contents = AdjustCodeCommandCore::remove_function(&modified_contents, "megaInit");
        modified_contents = AdjustCodeCommandCore::remove_struct_timeval(&modified_contents);
        modified_contents = AdjustCodeCommandCore::remove_enum_declaration(&modified_contents);
        // Todo: 定義前enumの削除
        // Todo: 定義されているenumを検出．プロトタイプ宣言しているところを削除

        modified_contents
    }
}

impl Command for AdjustCodeCommandCore {
    fn run(&self, use_docker: bool) -> Result<()> {
        let _ = use_docker;

        self.dataset.code_db.iter().for_each(|code: &CodeInfo| {
            if let Some(target) = &self.target {
                if !code.dir_name.eq(target) {
                    return;
                }
            }

            let dst_adj_dir_path = code.get_dst_adj_dir_path(&self.dataset.src_dir);
            // 出力先が存在しない場合はディレクトリ作成
            if !dst_adj_dir_path.exists() {
                let _ = fs::create_dir(&dst_adj_dir_path);
            }

            // Todo: Obfuscatedディレクトリの走査
            code.get_dst_dir_path(&self.dataset.src_dir)
                .read_dir()
                .unwrap()
                .into_iter()
                .filter_map(|result| result.ok())
                .for_each(|result| {
                    if !&result.path().extension().unwrap().eq("c") {
                        return;
                    }

                    let dst_adj_path: &PathBuf = &dst_adj_dir_path
                        .join(result.file_name())
                        .with_extension("c");

                    let adj_content =
                        AdjustCodeCommandCore::main(fs::read_to_string(&result.path()).unwrap());

                    let mut file = File::create(dst_adj_path).unwrap();
                    let _ = write!(file, "{}", adj_content);
                });
        });
        Ok(())
    }
}
