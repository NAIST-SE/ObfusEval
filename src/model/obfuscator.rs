use std::{
    fs::{self, File},
    io::BufReader,
    process,
    time::Duration,
};

use super::*;
use anyhow::Result;
use duct::{cmd, Expression};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tigress {
    pub name: String,
    execution_path: PathBuf,
    common_parameter: Vec<String>,
    #[serde(skip)]
    common_command: (String, Vec<String>),
    pub transformation_set: Vec<Obfuscation>,
}

impl Obfuscator for Tigress {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_transformation_names(&self) -> Vec<&String> {
        self.transformation_set
            .iter()
            .map(|t| t.get_display_name())
            .collect()
    }

    fn obfuscate_by_all_obfuscation(
        &self,
        src_path: &PathBuf,
        dst_dir_path: &PathBuf,
        function_name: &str,
        pb: &Option<ProgressBar>,
    ) -> Result<()> {
        if !dst_dir_path.exists() {
            fs::create_dir(&dst_dir_path)?;
        }

        // todo: エラーを集約し，要素が一つでもある場合はエラーを返すようにする
        let _: Vec<_> = self
            .transformation_set
            .par_iter()
            .map(|obfuscation| {
                self.obfuscate(src_path, dst_dir_path, function_name, obfuscation, pb)
            })
            .collect();

        if let Some(pb) = pb {
            let pb_style =
                ProgressStyle::with_template("   [{elapsed_precise}] {prefix} {msg}").unwrap();
            pb.set_style(pb_style);
            pb.finish_with_message("obfuscated.");
        }

        Ok(())
    }

    fn obfuscate(
        &self,
        src_path: &PathBuf,
        dst_dir_path: &PathBuf,
        function_name: &str,
        obfuscation: &Obfuscation,
        pb: &Option<ProgressBar>,
    ) -> Result<()> {
        let dst_path: &PathBuf = &dst_dir_path
            .join(&obfuscation.get_display_name())
            .with_extension("c");
        if dst_path.exists() {
            return Ok(());
        }

        if let Some(pb) = pb {
            pb.enable_steady_tick(Duration::from_millis(100));
            pb.set_message(format!("{:<10}", &obfuscation.get_display_name()));
        }
        let obfuscation_param: Vec<String> = obfuscation.get_obfuscate_parameter(function_name);
        let io_param: Vec<String> = self.get_io_parameter(&src_path, &dst_path);
        let args = [self.common_command.1.clone(), obfuscation_param, io_param].concat();

        // todo: コードが生成されているかどうかを判定する
        let command: Expression = cmd(self.common_command.0.clone(), args);
        let _output = command.unchecked().stdout_null().stderr_capture().run();
        // todo: docker-compose.ymlが配置されているディレクトリ上でないと動作しないぽい
        if !dst_path.exists() {
            eprintln!("docker-compose.yml が配置されているディレクトリ上で実行してください");
            process::exit(1);
        }

        if let Some(pb) = pb {
            pb.inc(1);
        }

        Ok(())
    }
}

impl Tigress {
    pub fn new(path: &PathBuf, docker_compose_file: &Option<PathBuf>) -> Self {
        let file: File = File::open(path).unwrap();
        let rdr: BufReader<File> = BufReader::new(file);
        let mut model: Tigress = serde_json::from_reader(rdr).unwrap();

        model.set_common_command(docker_compose_file);
        model
    }

    fn set_common_command(&mut self, docker_compose_file: &Option<PathBuf>) {
        let exec_path: &str = self.execution_path.to_str().unwrap();

        self.common_command = match docker_compose_file {
            Some(config_file) => (
                String::from("docker"),
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
            None => (exec_path.to_string(), self.common_parameter.clone()),
        }
    }

    fn get_io_parameter(&self, src_path: &PathBuf, dst_path: &PathBuf) -> Vec<String> {
        vec![
            format!(
                "-o {}",
                dst_path.with_extension("elf").as_path().to_str().unwrap()
            ),
            src_path.clone().into_os_string().into_string().unwrap(),
            format!("--out={}", dst_path.to_string_lossy()),
        ]
    }
}

/// 難読化手法を管理する構造体．
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Obfuscation {
    name: String,
    display_name: String,
    parameter: Vec<String>,
}

impl Obfuscation {
    fn get_display_name(&self) -> &String {
        &self.display_name
    }

    fn get_obfuscate_parameter(&self, function_name: &str) -> Vec<String> {
        self.parameter
            .join(" ")
            .replace("--Functions=*", &format!("--Functions={}", function_name))
            .split_whitespace()
            .map(|x| x.to_string())
            .collect()
    }
}
