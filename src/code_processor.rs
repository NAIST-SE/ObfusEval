use anyhow::Result;
use compiler::Compiler;
use duct::cmd;
use obfuscator::Obfuscator;
use serde::Deserialize;

use crate::Load;

pub mod compiler;
pub mod obfuscator;
mod parser;

pub trait Executor {
    fn label(&self) -> &str;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn bin_path(&self) -> &str;
    fn bin_args(&self) -> Option<Vec<&str>>;

    fn execute(
        &self,
        args: &[String],
        working_directory: &std::path::PathBuf,
        dry_run: &bool,
    ) -> Result<()> {
        let program = self.bin_path();

        if *dry_run {
            println!("{} {}", &program, &args.join(" "));
        } else {
            println!("{} {}", &program, &args.join(" "));
            cmd(program, args).dir(&working_directory).read().unwrap();
        }
        Ok(())
    }
}

pub trait LoadCompiler {
    fn to_compiler(self, path: &std::path::PathBuf) -> anyhow::Result<Box<dyn Compiler>>;
}

pub trait LoadObfuscator {
    fn to_obfuscator(self, path: &std::path::PathBuf) -> anyhow::Result<Box<dyn Obfuscator>>;
}

trait Organizer {
    fn organize(src_path: &std::path::PathBuf, dst_path: &std::path::PathBuf)
        -> anyhow::Result<()>;
}

#[derive(Debug, Deserialize)]
pub struct ExecutorInfo {
    label: String,
    pub name: String,
    pub version: String,
    binary_path: String,
    binary_args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct ExecutorInfoSerializeModel {
    label: String,
    name: String,
    version: String,
    binary: Vec<String>,
}

impl Load for ExecutorInfo {
    fn load(path: &std::path::PathBuf) -> anyhow::Result<Self> {
        let json = std::fs::File::open(path)?;
        let value: ExecutorInfoSerializeModel = serde_json::from_reader(json)?;

        Ok(Self {
            label: value.label,
            name: value.name,
            version: value.version,
            binary_path: value.binary[0].clone(),
            binary_args: Some(value.binary[1..].to_vec()),
        })
    }
}
