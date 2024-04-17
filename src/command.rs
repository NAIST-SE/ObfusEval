use std::path::PathBuf;

use anyhow::Result;
use clap::{arg, Parser};

pub mod adjust_code;
pub mod obfuscate;

pub trait Command {
    fn run(&self, use_docker: bool) -> Result<()>;
}

#[derive(Parser)]
pub struct SampleCommand {
    #[arg(
        long = "dataset",
        help = "Path to the json file that manages dataset information"
    )]
    pub dataset_path: PathBuf,

    #[arg(help = "Target source code name")]
    pub target: String,
}
