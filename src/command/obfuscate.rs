use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use std::path::PathBuf;

use clap::{command, Parser};

use crate::model::{
    dataset::{Dataset, DatasetSerealizeModel},
    obfuscator::Tigress,
    DatasetHandler, Obfuscator,
};

use super::Command;

#[derive(Parser)]
#[command(about = "Obfuscate the code in the dataset")]
pub struct Args {
    #[arg(help = "Path to the json file that manages dataset repository")]
    pub dataset_json_path: PathBuf,

    #[arg(long = "target", help = "Target source code name")]
    pub target: Option<String>,
}

pub struct ObfuscateCommand {
    dataset: Dataset<Tigress>,
    target: Option<String>,
}

impl From<Args> for ObfuscateCommand {
    fn from(args: Args) -> Self {
        Self {
            dataset: Dataset::from(DatasetSerealizeModel::new(&args.dataset_json_path)),
            target: args.target,
        }
    }
}

impl Command for ObfuscateCommand {
    fn run(&self) -> Result<()> {
        if let Some(target) = &self.target {
            if let Some(code) = self
                .dataset
                .code_db
                .iter()
                .find(|x| x.target.starts_with(target))
            {
                let mp: MultiProgress = MultiProgress::new();
                let pb_style: ProgressStyle = ProgressStyle::with_template(
                    "{spinner:.green} [{elapsed_precise}] {prefix} {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
                    )
                    .unwrap()
                    .progress_chars("#>-");

                let _: Vec<_> = self
                    .dataset
                    .obfuscator_db
                    .iter()
                    .map(|obfuscator| {
                        let obfuscation_len: u64 =
                            obfuscator.get_transformation_names().len() as u64;
                        let pb = mp.add(ProgressBar::new(obfuscation_len));
                        pb.set_style(pb_style.clone());
                        pb.set_prefix(format!("{:<10}", code.target));

                        obfuscator.obfuscate_by_all_obfuscation(
                            &code.src_path,
                            &Dataset::<Tigress>::get_dst_dir_path(code),
                            &code.function,
                            &Some(pb),
                        )
                    })
                    .collect();
            }
        } else {
            self.dataset.obfuscate_each_obfuscator()?;
        }

        // println!("---\nAutomatic adjustment of obfuscated code to a compilable form.");
        // let _ = self.to_adjust_code_command().run();
        // println!("Complete.");

        Ok(())
    }
}
