use anyhow::Result;
use duct::cmd;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use thiserror::Error;
use users::{get_current_gid, get_current_uid};

#[derive(Debug, Error)]
pub enum BuilderError {
    // #[error("RunDockerError: {0}")]
    #[error("{0}")]
    RunDockerServiceError(String),
}

#[derive(Debug)]
pub struct DockerService<'a> {
    name: &'a String,
    exec_command: &'a String,
}

impl DockerService<'_> {
    pub fn run(&self, add_args: Vec<&str>) -> Result<()> {
        let id: String = format!("{}:{}", get_current_uid(), get_current_gid());
        let pre_args: Vec<&str> = vec![
            "exec",
            "--user",
            id.as_str(),
            "-T",
            self.name.as_str(),
            self.exec_command.as_str(),
        ];

        let args: Vec<&str> = vec![pre_args, add_args]
            .into_iter()
            .flat_map(|x| x)
            .collect();

        let output = cmd("docker-compose", &args)
            .unchecked()
            .stderr_capture()
            .run()?;

        if !&output.status.success() {
            return Err(BuilderError::RunDockerServiceError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            )
            .into());
        }

        Ok(())
    }

    pub fn run_with_top_args(&self, common_flag: Vec<&str>, add_args: Vec<&str>) -> Result<()> {
        let args: Vec<&str> = vec![common_flag, add_args]
            .into_iter()
            .flat_map(|x| x)
            .collect();

        self.run(args)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Obfuscator {
    pub name: String,
    compiler: String,
    pub common_flag: Option<Vec<String>>,
    pub is_bin_only: bool,
    pub transformations: Vec<Transformation>,
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


#[derive(Deserialize, Serialize, Debug)]
pub struct Transformation {
    pub name: String,
    pub display: String,
    flag: Vec<String>,
}

impl Transformation {
    pub fn flag(&self) -> String {
        self.flag.join(" ")
    }
}
