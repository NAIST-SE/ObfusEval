use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::PathBuf;

use clap::Parser;

use crate::model::dataset::Dataset;

use super::Command;

#[derive(Parser)]
pub struct Args {
    #[arg(
        long = "use-docker-compose",
        help = "Execute command via docker service",
        default_value_t = false
    )]
    pub use_docker_compose: bool,

    #[arg(help = "Path to the json file that manages dataset repository")]
    pub dataset_json_path: PathBuf,

    #[arg(long = "target", help = "Target source code name")]
    pub target: Option<String>,
}

pub struct ObfuscateCommand {
    use_docker_compose: bool,
    dataset: Dataset,
    target: Option<String>,
}

impl ObfuscateCommand {
    pub fn new(args: Args) -> Self {
        Self {
            use_docker_compose: args.use_docker_compose,
            dataset: Dataset::new(&args.dataset_json_path),
            target: args.target,
        }
    }
}

impl Command for ObfuscateCommand {
    fn run(&self) -> Result<()> {
        let bar = ProgressBar::new(self.dataset.code_db.len() as u64);
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap(),
        );

        self.dataset
            .obfuscator_db
            .par_iter()
            .for_each(|obfuscator| {
                for code in self.dataset.code_db.iter() {
                    if let Some(target) = &self.target {
                        if !code.dir_name.eq(target) {
                            continue;
                        }
                    }

                    let _ = obfuscator.obfuscate(&self.dataset, &code, &self.use_docker_compose);
                    bar.inc(1);
                    bar.set_message(format!("{}", code.target));
                }
            });

        Ok(())
    }
}
