use std::fs;

use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Code {
    dir_name: String,
    pub target: String,
    pub function: String,
    #[serde(skip)]
    pub src_path: PathBuf,
}

impl Code {
    pub fn set_src_path(&mut self, src_dir: &PathBuf) {
        self.src_path = fs::canonicalize(src_dir.join(&self.dir_name).join(&self.target)).unwrap();
    }
}
