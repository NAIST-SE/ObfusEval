use std::fs;

use self::{code::CodeInfo, obfuscator::Obfuscator};

use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct DatasetSerealizeModel {
    name: String,
    src_dir: PathBuf,
    obfuscator_db: Vec<PathBuf>,
    docker_compose_file: Option<PathBuf>,
    code_db: Vec<CodeInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dataset {
    name: String,
    pub src_dir: PathBuf,
    pub obfuscator_db: Vec<Obfuscator>,
    pub docker_compose_file: Option<PathBuf>,
    pub code_db: Vec<CodeInfo>,
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

        let config_file: Option<PathBuf> = if let Some(f) = model.docker_compose_file {
            Some(
                fs::canonicalize(dataset_dir_path.join(&f))
                    .expect(&format!("[Model::Dataset] File not found: {:?}", f)),
            )
        } else {
            None
        };

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
            docker_compose_file: config_file,
            code_db: model.code_db,
        }
    }
}
