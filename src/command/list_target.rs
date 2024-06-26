use std::path::PathBuf;

use anyhow::Result;

use clap::Parser;

use crate::model::{
    dataset::{Dataset, DatasetSerealizeModel},
    obfuscator::Tigress,
};

use super::Command;

#[derive(Parser)]
#[command(about = "Display obfuscations.")]
pub struct Args {
    #[arg(help = "Path to the json file that manages dataset repository")]
    pub dataset_json_path: PathBuf,
}

pub struct ListTargetCommand {
    dataset: Dataset<Tigress>,
}

impl From<Args> for ListTargetCommand {
    fn from(args: Args) -> Self {
        Self {
            dataset: Dataset::from(DatasetSerealizeModel::new(&args.dataset_json_path)),
        }
    }
}

impl Command for ListTargetCommand {
    fn run(&self) -> Result<()> {
        let targets: String = self
            .dataset
            .code_db
            .iter()
            .map(|x| x.dir_name.clone())
            .collect::<Vec<String>>()
            .join(" ");

        println!("{}", targets);

        Ok(())
    }
}
