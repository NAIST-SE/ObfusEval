use self::code::CodeInfo;

use super::*;

/// 難読化手法を管理する構造体．
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Obfuscation {
    name: String,
    pub display_name: String,
    parameter: Vec<String>,
}

impl Obfuscation {
    pub fn get_obfuscate_parameter(&self, code: &CodeInfo) -> Vec<String> {
        self.parameter
            .join(" ")
            .replace("--Functions=*", &format!("--Functions={}", code.function))
            .split_whitespace()
            .map(|x| x.to_string())
            .collect()
    }
}
