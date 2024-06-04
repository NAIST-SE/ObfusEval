use std::{fs, time::Duration};

use self::{code::CodeInfo, dataset::Dataset, obfuscation::Obfuscation};
use super::*;
use anyhow::Result;
use duct::{cmd, Expression};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub trait ObfuscatorTrait {
    fn obfuscate(&self, dataset: &Dataset, code: &CodeInfo, target_pb: &ProgressBar) -> Result<()>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Obfuscator {
    pub name: String,
    execution_path: PathBuf,
    common_parameter: Vec<String>,
    pub transformation_set: Vec<Obfuscation>,
}

impl ObfuscatorTrait for Obfuscator {
    fn obfuscate(&self, dataset: &Dataset, code: &CodeInfo, target_pb: &ProgressBar) -> Result<()> {
        // 難読化のコマンドと共通パラメータを設定
        let (command, obfuscate_param): (&str, Vec<String>) =
            self.make_common_command(&dataset.docker_compose_file);

        let src_path: PathBuf = code.get_src_path(&dataset.src_dir);
        let dst_dir_path: PathBuf = code.get_dst_dir_path(&dataset.src_dir);
        // 出力先が存在しない場合はディレクトリ作成
        if !dst_dir_path.exists() {
            let _ = fs::create_dir(&dst_dir_path);
        }

        let _: Vec<_> = self
            .transformation_set
            .par_iter()
            .map(|obfuscation| {
                let dst_path: &PathBuf = &dst_dir_path
                    .join(&obfuscation.display_name)
                    .with_extension("c");
                if dst_path.exists() {
                    return;
                }

                target_pb.enable_steady_tick(Duration::from_millis(100));
                target_pb.set_message(format!("{:<10}", &obfuscation.display_name));

                let obfuscation_param: Vec<String> = obfuscation.get_obfuscate_parameter(code);
                let io_param: Vec<String> = Obfuscator::get_io_parameter(&src_path, &dst_path);
                let args = [obfuscate_param.clone(), obfuscation_param, io_param].concat();

                // 成否判定(コードが生成されていれば，とりあえずOKとする)
                let command: Expression = cmd(command, args);
                let _output = command.unchecked().stdout_null().stderr_capture().run();

                target_pb.inc(1);
            })
            .collect();

        let pb_style =
            ProgressStyle::with_template("   [{elapsed_precise}] {prefix} {msg}").unwrap();
        target_pb.set_style(pb_style);
        target_pb.finish_with_message("obfuscated.");

        Ok(())
    }
}

impl Obfuscator {
    pub fn new(path: &PathBuf) -> Self {
        let file: File = File::open(path).unwrap();
        let rdr: BufReader<File> = BufReader::new(file);
        serde_json::from_reader(rdr).unwrap()
    }

    fn make_common_command(&self, docker_compose_file: &Option<PathBuf>) -> (&str, Vec<String>) {
        let exec_path: &str = self.execution_path.to_str().unwrap();

        match docker_compose_file {
            Some(config_file) => (
                "docker",
                vec![
                    "compose",
                    "--file",
                    config_file.to_str().unwrap(),
                    "run",
                    "--rm",
                    &self.name,
                    exec_path,
                ]
                .iter()
                .map(|s| s.to_string())
                .chain(self.common_parameter.iter().map(|s| s.to_string()))
                .collect(),
            ),
            None => (exec_path, self.common_parameter.clone()),
        }
    }

    fn get_io_parameter(src_path: &PathBuf, dst_path: &PathBuf) -> Vec<String> {
        vec![
            format!(
                "-o {}",
                dst_path.with_extension("elf").as_path().to_str().unwrap()
            ),
            src_path.clone().into_os_string().into_string().unwrap(),
            format!("--out={}", dst_path.to_string_lossy()),
        ]
    }

    pub fn get_obfuscation_display_name(&self) -> Vec<String> {
        self.transformation_set
            .iter()
            .map(|t| t.display_name.clone())
            .collect()
    }
}
