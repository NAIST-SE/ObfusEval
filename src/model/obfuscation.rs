use super::*;

/// 難読化手法を管理する構造体．
#[derive(Debug, Deserialize, Serialize)]
pub struct Obfuscation {
    name: String,
    pub display_name: String,
    parameter: Vec<String>,
}

impl Obfuscation {
    pub fn get_parameter(&self) -> String {
        self.parameter.join(" ")
    }
}
