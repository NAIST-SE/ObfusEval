use self::obfuscation::Obfuscation;
use super::*;

/// 難読化ツールを管理する構造体．
#[derive(Debug, Deserialize, Serialize)]
pub struct Obfuscator {
    name: String,
    execution_path: PathBuf,
    common_parameter: Vec<String>,
    pub transformation_set: Vec<Obfuscation>,
}

impl Obfuscator {
    pub fn new(path: &PathBuf) -> Self {
        let file: File = File::open(path).unwrap();
        let rdr: BufReader<File> = BufReader::new(file);
        serde_json::from_reader(rdr).unwrap()
    }

    pub fn make_common_command(&self, use_docker_compose: bool) -> (&str, Vec<&str>) {
        let exec_path: &str = self.execution_path.to_str().unwrap();
        match use_docker_compose {
            true => (
                "docker",
                [
                    vec!["compose", "run", "--rm", &self.name, exec_path],
                    self.get_common_parameter(),
                ]
                .concat(),
            ),
            false => (exec_path, self.get_common_parameter()),
        }
    }

    fn get_common_parameter(&self) -> Vec<&str> {
        self.common_parameter
            .iter()
            .map(|s: &String| s.as_str())
            .collect()
    }
}
