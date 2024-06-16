use std::path::PathBuf;

use anyhow::Result;

use clap::Parser;

use crate::model::{
    dataset::{Dataset, DatasetSerealizeModel},
    obfuscator::Tigress,
    Obfuscator,
};

use super::Command;

#[derive(Parser)]
#[command(about = "Display obfuscations.")]
pub struct Args {
    #[arg(help = "Path to the json file that manages dataset repository")]
    pub dataset_json_path: PathBuf,
}

pub struct ListObfuscationCommand {
    dataset: Dataset<Tigress>,
}

impl From<Args> for ListObfuscationCommand {
    fn from(args: Args) -> Self {
        Self {
            dataset: Dataset::from(DatasetSerealizeModel::new(&args.dataset_json_path)),
        }
    }
}

impl Command for ListObfuscationCommand {
    fn run(&self) -> Result<()> {
        let obfuscation_set: String = self
            .dataset
            .obfuscator_db
            .iter()
            .flat_map(|o| o.get_transformation_names())
            .collect::<Vec<&String>>()
            .into_iter()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
            .join(" ");

        println!("{}", obfuscation_set);

        Ok(())
    }
}
