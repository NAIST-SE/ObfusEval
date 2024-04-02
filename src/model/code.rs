use super::*;

/// コードと難読化対象となる関数名を管理する構造体．
#[derive(Debug, Deserialize, Serialize)]
pub struct Code {
    pub dir_name: String,
    pub target: String,
    pub function: String,
}
