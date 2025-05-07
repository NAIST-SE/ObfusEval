use serde::Deserialize;

use crate::bundle::CompilerOption;

use super::{Executor, ExecutorInfo, LoadCompiler};

pub mod gcc;
use gcc::Gcc;

pub trait Compiler: Executor {
    fn new(executor: ExecutorInfo, value: CompilerSerializeModel) -> Self
    where
        Self: Sized;

    fn flags(&self, option: &CompilerOption) -> Vec<String>;

    fn args(
        &self,
        source: &[std::path::PathBuf],
        target: &std::path::PathBuf,
        option: &CompilerOption,
    ) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        if let Some(bin_args) = self.bin_args() {
            bin_args.iter().for_each(|x| result.push(x.to_string()));
        }

        source
            .iter()
            .for_each(|p| result.push(p.display().to_string()));

        result.push("-o".to_string());
        result.push(target.display().to_string());

        self.flags(option)
            .iter()
            .for_each(|s| result.push(s.to_string()));

        result
    }

    fn compile(
        &self,
        working_directory: &std::path::PathBuf,
        source: &[std::path::PathBuf],
        target: &std::path::PathBuf,
        option: &CompilerOption,
        dry_run: &bool,
    ) -> anyhow::Result<()> {
        let args = self.args(source, target, option);
        self.execute(&args, working_directory, dry_run)
    }
}

impl std::fmt::Debug for dyn Compiler {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Deserialize)]
pub struct CompilerSerializeModel {
    target_language: String,
    target_arch: String,
    flags: Vec<String>,
    coverage_flags: Vec<String>,
    test_flags: Vec<String>,
}

impl LoadCompiler for ExecutorInfo {
    fn to_compiler(self, path: &std::path::PathBuf) -> anyhow::Result<Box<dyn Compiler>> {
        let json = std::fs::File::open(path)?;
        let value: CompilerSerializeModel = serde_json::from_reader(json)?;

        if self.name == "gcc" {
            return Ok(Box::new(Gcc::new(self, value)));
        }
        Err(anyhow::anyhow!("Unsupported compiler: {}", self.name))
    }
}
