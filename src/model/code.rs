use super::*;

/// コードと難読化対象となる関数名を管理する構造体．
#[derive(Debug, Deserialize, Serialize)]
pub struct Code {
    pub dir_name: String,
    pub target: String,
    pub function: String,
}

impl Code {
    pub fn get_src_path(&self, dir_path: &PathBuf) -> PathBuf {
        dir_path.join(&self.dir_name).join(&self.target)
    }

    pub fn get_dst_dir_path(&self, dir_path: &PathBuf) -> PathBuf {
        dir_path.join(&self.dir_name).join("obfuscated/")
    }
}
