use std::fs;

use self::{code::Code, obfuscator::Obfuscator};

use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct DatasetSerealizeModel {
    name: String,
    src_dir: PathBuf,
    obfuscator_db: Vec<PathBuf>,
    code_db: Vec<Code>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dataset {
    name: String,
    pub src_dir: PathBuf,
    pub obfuscator_db: Vec<Obfuscator>,
    pub code_db: Vec<Code>,
    is_docker_allowed: bool,
}

impl DatasetSerealizeModel {
    pub fn new(path: &PathBuf) -> Self {
        let file: File = File::open(path).unwrap();
        let rdr: BufReader<File> = BufReader::new(file);
        serde_json::from_reader(rdr).unwrap()
    }
}

impl Dataset {
    pub fn new(path: &PathBuf) -> Self {
        let dataset_file_path = fs::canonicalize(path).unwrap();
        let dataset_dir_path = dataset_file_path.parent().unwrap();

        let model: DatasetSerealizeModel = DatasetSerealizeModel::new(&path);
        Dataset {
            name: model.name,
            src_dir: dataset_dir_path.join(model.src_dir),
            obfuscator_db: model
                .obfuscator_db
                .iter()
                .map(|p| {
                    let obfuscator_dp_path: PathBuf =
                        fs::canonicalize(dataset_dir_path.join(p)).unwrap();
                    Obfuscator::new(&obfuscator_dp_path)
                })
                .collect(),
            code_db: model.code_db,
            is_docker_allowed: dataset_dir_path.join("docker-compose.yml").exists(),
        }
    }
}
