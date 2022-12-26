use anyhow::{anyhow, Result};
use duct::cmd;
use users::{get_current_gid, get_current_uid};

use super::BuilderError;

#[derive(Debug)]
pub struct DockerService<'a> {
    pub name: &'a String,
    pub exec_command: &'a String,
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
                anyhow!(String::from_utf8_lossy(&output.stderr).to_string()).into(),
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