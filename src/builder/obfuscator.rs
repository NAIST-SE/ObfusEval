use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;

use super::service::DockerService;

#[derive(Deserialize, Serialize, Debug)]
pub struct Obfuscator {
    pub name: String,
    compiler: String,
    pub common_flag: Option<Vec<String>>,
    pub is_bin_only: bool,
    pub transformations: Vec<Transformation>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transformation {
    pub name: String,
    display: String,
    flag: Vec<String>,
}

impl Obfuscator {
    pub fn new(p: PathBuf) -> Result<Obfuscator> {
        let path: PathBuf = p.canonicalize()?;
        let file: File = File::open(path)?;
        Ok(serde_json::from_reader(file).unwrap())
    }

    pub fn transformations_name(&self) -> Vec<&String> {
        self.transformations.iter().map(|t| &t.name).collect()
    }

    pub fn compiler(&self) -> DockerService {
        DockerService {
            name: &self.name,
            exec_command: &self.compiler,
        }
    }

    pub fn run(&self, args: Vec<&str>) -> Result<()> {
        let tool = DockerService {
            name: &self.name,
            exec_command: &"obfuscate".to_string(),
        };

        if let Some(common_flag) = &self.common_flag {
            tool.run_with_top_args(
                common_flag.iter().map(AsRef::as_ref).collect::<Vec<&str>>(),
                args,
            )?;
        } else {
            tool.run(args)?;
        }
        Ok(())
    }
}

impl Transformation {
    pub fn short_name(&self) -> String {
        self.display.clone()
    }

    pub fn flag(&self) -> String {
        self.flag.join(" ")
    }
}