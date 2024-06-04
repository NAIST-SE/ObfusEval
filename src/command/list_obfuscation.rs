use std::path::PathBuf;

use anyhow::Result;

use clap::Parser;

use crate::model::dataset::Dataset;

use super::Command;

#[derive(Parser)]
pub struct Args {
    #[arg(help = "Path to the json file that manages dataset repository")]
    pub dataset_json_path: PathBuf,
}

pub struct ListObfuscationCommand {
    pub dataset: Dataset,
}

impl ListObfuscationCommand {
    pub fn new(args: Args) -> Self {
        Self {
            dataset: Dataset::new(&args.dataset_json_path),
        }
    }
}

impl Command for ListObfuscationCommand {
    fn run(&self) -> Result<()> {
        let obfuscation_set: String = self
            .dataset
            .obfuscator_db
            .iter()
            .flat_map(|o| o.get_obfuscation_display_name())
            .collect::<Vec<String>>()
            .join(" ");

        println!("{}", obfuscation_set);

        Ok(())
    }
}
