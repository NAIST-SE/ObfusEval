use super::*;

/// コードと難読化対象となる関数名を管理する構造体．
#[derive(Deserialize, Serialize)]
pub struct Code {
    pub name: String,
    pub path: String,
    pub function: String,
}

impl Code {
    pub fn new(path: &PathBuf) -> Vec<Self> {
        let file: File = File::open(path).unwrap();
        let rdr: BufReader<File> = BufReader::new(file);
        serde_json::from_reader(rdr).unwrap()
    }
}
