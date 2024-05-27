use anyhow::Result;
use clap::Parser;
use std::fs;
use std::io::Write;
use std::{fs::File, path::PathBuf};

use crate::model::code_adjuster::{CodeAdjuster, TigressCodeAdjuster};
use crate::model::dataset::Dataset;

use super::Command;

#[derive(Parser)]
pub struct Args {
    #[arg(help = "Path to the json file that manages dataset repository")]
    pub dataset_json_path: PathBuf,

    #[arg(long = "target", help = "Target source code name")]
    pub target: Option<String>,
}

pub struct AdjustCodeCommand {
    dataset: Dataset,
    target: Option<String>,
}

impl AdjustCodeCommand {
    pub fn new(args: Args) -> Self {
        Self {
            dataset: Dataset::new(&args.dataset_json_path),
            target: args.target,
        }
    }
}

impl Command for AdjustCodeCommand {
    fn run(&self) -> Result<()> {
        for code in self.dataset.code_db.iter() {
            if let Some(target) = &self.target {
                if !code.dir_name.eq(target) {
                    continue;
                }
            }

            let dst_adj_dir_path = code.get_dst_adj_dir_path(&self.dataset.src_dir);
            // 出力先が存在しない場合はディレクトリ作成
            if !dst_adj_dir_path.exists() {
                let _ = fs::create_dir(&dst_adj_dir_path);
            }

            // Todo: Obfuscatedディレクトリの走査
            code.get_dst_dir_path(&self.dataset.src_dir)
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

                    let adjuster = TigressCodeAdjuster::new(result.path());

                    if let Ok(adjusted_source_code) = adjuster.adjust_code() {
                        let mut file = File::create(dst_adj_path).unwrap();
                        let _ = write!(file, "{}", adjusted_source_code);
                    };
                });
        }

        Ok(())
    }
}
