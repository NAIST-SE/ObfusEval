// 難読化されたコードをコンパイル可能な形sに調整するロジック
use anyhow::{Context, Result};

use std::{fs, path::PathBuf, str::Lines};

pub trait CodeAdjuster {
    fn adjust_code(&self) -> Result<String>;
}

pub struct TigressCodeAdjuster {
    file_path: PathBuf,
    source_code: String,
    adjusted_source_code: Option<String>,
}

impl CodeAdjuster for TigressCodeAdjuster {
    fn adjust_code(&self) -> Result<String> {
        // もっと丁寧にやるべきだが，とりあえずTigressでは動くためOKとする
        // Todo: 定義前enumの削除
        // // Todo: 定義されているenumを検出．プロトタイプ宣言しているところを削除
        let result = self
            .remove_extern_function()
            .remove_function("main")
            .remove_function("megaInit")
            .remove_struct_timeval()
            .remove_enum_declaration()
            .insert_cstdlib();

        result
            .adjusted_source_code
            .with_context(|| format!("Adjust Error"))
    }
}

impl TigressCodeAdjuster {
    pub fn new(file_path: PathBuf) -> Self {
        let source_code = fs::read_to_string(&file_path).unwrap();

        TigressCodeAdjuster {
            file_path: file_path,
            source_code: source_code,
            adjusted_source_code: None,
        }
    }
}

impl TigressCodeAdjuster {
    fn get_target_code(&self) -> Lines<'_> {
        match &self.adjusted_source_code {
            Some(target) => target.lines(),
            None => self.source_code.lines(),
        }
    }

    fn set_adjusted_code(&self, adjusted_code: String) -> Self {
        TigressCodeAdjuster {
            file_path: self.file_path.clone(),
            source_code: self.source_code.clone(),
            adjusted_source_code: Some(adjusted_code),
        }
    }

    fn remove_extern_function(&self) -> Self {
        let mut adjusted_code = String::new();

        let mut scope_num: isize = -1;
        let mut function_code = String::new();
        for line in self.get_target_code() {
            if line.trim().starts_with("extern") {
                scope_num = 0;
            }

            if scope_num < 0 {
                adjusted_code.push_str(line);
                adjusted_code.push('\n');
                continue;
            }

            if line.contains(") ;") {
                scope_num -= 1;
            }

            function_code.push_str(line);
            function_code.push('\n');
        }

        // dbg!(function_code);
        self.set_adjusted_code(adjusted_code)
    }

    fn insert_cstdlib(&self) -> Self {
        let mut adjusted_code = String::new();

        let mut is_cstdlib_included = false;
        for line in self.get_target_code() {
            if line.contains("malloc") || line.contains("rand") {
                if is_cstdlib_included {
                    continue;
                }
                adjusted_code.insert_str(0, "#include <cstdlib>\n");
                is_cstdlib_included = true;
            }
            adjusted_code.push_str(line);
            adjusted_code.push_str("\n");
        }

        self.set_adjusted_code(adjusted_code)
    }

    fn remove_function(&self, name: &str) -> Self {
        let mut adjusted_code = String::new();

        let function_name = format!("{}(", name);
        let mut scope_num: isize = -1;
        let mut function_code = String::new();
        for line in self.get_target_code() {
            if scope_num < 0 && line.contains(&function_name) {
                scope_num = 0;
            }

            if scope_num < 0 {
                adjusted_code.push_str(line);
                adjusted_code.push('\n');
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

        self.set_adjusted_code(adjusted_code)
    }

    fn remove_struct_timeval(&self) -> Self {
        let mut adjusted_code = String::new();

        let mut scope_num: isize = -1;
        let mut function_code = String::new();
        for line in self.get_target_code() {
            if line.contains(&"struct timeval {") {
                scope_num = 0;
            }

            if scope_num < 0 {
                adjusted_code.push_str(line);
                adjusted_code.push('\n');
                continue;
            }

            if line.contains("};") {
                scope_num -= 1;
            }

            function_code.push_str(line);
            function_code.push('\n');
        }

        // dbg!(function_code);
        self.set_adjusted_code(adjusted_code)
    }

    fn remove_enum_declaration(&self) -> Self {
        let mut adjusted_code = String::new();

        for line in self.get_target_code() {
            if line.trim().starts_with("enum") && line.ends_with(";") {
                continue;
            }

            adjusted_code.push_str(line);
            adjusted_code.push('\n');
            continue;
        }

        // dbg!(function_code);
        self.set_adjusted_code(adjusted_code)
    }
}
