use anyhow::Result;
use clap::Parser;
use std::fs;

use std::path::PathBuf;

use crate::model::code_organizer::TigressCodeOrganizer;
use crate::model::dataset::{Dataset, DatasetSerealizeModel};
use crate::model::obfuscator::Tigress;
use crate::model::CodeOrganizer;

use super::obfuscate::ObfuscateCommand;
use super::Command;

#[derive(Parser)]
#[command(about = "Adjust obfuscated code to a compilable form.")]
pub struct Args {
    #[arg(help = "Path to the json file that manages dataset repository")]
    pub dataset_json_path: PathBuf,

    #[arg(long = "target", help = "Target source code name")]
    pub target: Option<String>,
}

pub struct AdjustCodeCommand {
    pub dataset: Dataset<Tigress>,
    pub target: Option<String>,
}

impl From<Args> for AdjustCodeCommand {
    fn from(args: Args) -> Self {
        Self {
            dataset: Dataset::from(DatasetSerealizeModel::new(&args.dataset_json_path)),
            target: args.target,
        }
    }
}

impl From<&ObfuscateCommand> for AdjustCodeCommand {
    fn from(cmd: &ObfuscateCommand) -> Self {
        Self {
            dataset: cmd.dataset.clone(),
            target: cmd.target.clone(),
        }
    }
}

impl Command for AdjustCodeCommand {
    fn run(&self) -> Result<()> {
        self.dataset.code_db.iter().for_each(|code| {
            if let Some(target) = &self.target {
                if !code.target.starts_with(target) {
                    return;
                }
            }

            let dst_adj_dir_path = &Dataset::<Tigress>::get_dst_adj_dir_path(code);
            if !dst_adj_dir_path.exists() {
                fs::create_dir(&dst_adj_dir_path).unwrap();
            }

            let dst_dir_path = &Dataset::<Tigress>::get_dst_dir_path(code);
            dst_dir_path
                .read_dir()
                .unwrap()
                .into_iter()
                .filter_map(|result| result.ok())
                .for_each(|result| {
                    if !&result.path().extension().unwrap().eq("c") {
                        return;
                    }

                    let dst_adj_path: &PathBuf = &dst_adj_dir_path
                        .join(result.file_name())
                        .with_extension("c");

                    let _ = TigressCodeOrganizer::organize(&result.path(), &dst_adj_path);
                });
        });

        Ok(())
    }
}
