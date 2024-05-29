use anyhow::Result;
use console::style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::PathBuf;

use clap::Parser;

use crate::{
    command::adjust_code::AdjustCodeCommand,
    model::{
        dataset::Dataset,
        obfuscator::{Obfuscator, ObfuscatorTrait},
    },
};

use super::Command;

#[derive(Parser)]
pub struct Args {
    #[arg(help = "Path to the json file that manages dataset repository")]
    pub dataset_json_path: PathBuf,

    #[arg(long = "target", help = "Target source code name")]
    pub target: Option<String>,
}

pub struct ObfuscateCommand {
    dataset: Dataset,
    target: Option<String>,
}

impl ObfuscateCommand {
    pub fn new(args: Args) -> Self {
        Self {
            dataset: Dataset::new(&args.dataset_json_path),
            target: args.target,
        }
    }

    fn to_adjust_code_command(&self) -> AdjustCodeCommand {
        AdjustCodeCommand {
            dataset: self.dataset.clone(),
            target: self.target.clone(),
        }
    }
}

impl Command for ObfuscateCommand {
    fn run(&self) -> Result<()> {
        self.dataset
            .obfuscator_db
            .iter()
            .enumerate()
            .for_each(|(idx, obfuscator)| {
                println!(
                    "{} Obfuscate::{}",
                    style(format!("[{}/1]", idx + 1)).bold().dim(),
                    style(format!("{}", obfuscator.name)).bold().dim(),
                );
                self.obfuscate_each_code(obfuscator);
            });

        println!("---\nAutomatic adjustment of obfuscated code to a compilable form.");
        let _ = self.to_adjust_code_command().run();
        println!("Complete.");

        Ok(())
    }
}

impl ObfuscateCommand {
    fn obfuscate_each_code(&self, obfuscator: &Obfuscator) {
        let mp = MultiProgress::new();
        let obfuscation_len = obfuscator.transformation_set.len() as u64;
        let pb_style = ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] {prefix} {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("#>-");

        let _: Vec<_> = self
            .dataset
            .code_db
            .par_iter()
            .map(|code| {
                if let Some(target) = &self.target {
                    if !code.dir_name.eq(target) {
                        return;
                    }
                }

                let pb = mp.add(ProgressBar::new(obfuscation_len));
                pb.set_style(pb_style.clone());
                pb.set_prefix(format!("{:<10}", code.target));

                let _ = obfuscator.obfuscate(&self.dataset, &code, &pb);
            })
            .collect();

        // mp.clear().unwrap();
    }
}
