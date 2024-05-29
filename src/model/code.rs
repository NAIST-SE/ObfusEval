use super::*;

/// コードと難読化対象となる関数名を管理する構造体．
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeInfo {
    pub dir_name: String,
    pub target: String,
    pub function: String,
}

impl CodeInfo {
    pub fn get_src_path(&self, dir_path: &PathBuf) -> PathBuf {
        dir_path.join(&self.dir_name).join(&self.target)
    }

    pub fn get_dst_dir_path(&self, dir_path: &PathBuf) -> PathBuf {
        dir_path.join(&self.dir_name).join("obfuscated_raw/")
    }

    pub fn get_dst_adj_dir_path(&self, dir_path: &PathBuf) -> PathBuf {
        dir_path.join(&self.dir_name).join("obfuscated/")
    }
}
