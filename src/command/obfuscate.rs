use anyhow::Result;
use duct::{cmd, Expression};
use std::{fs, path::PathBuf};

use clap::Parser;

use crate::model::{
    code::Code, dataset::Dataset, obfuscation::Obfuscation, obfuscator::Obfuscator,
};

use super::Command;

#[derive(Parser)]
pub struct ObfuscateCommand {
    #[arg(help = "Path to the json file that manages dataset repository")]
    pub dataset_json_path: PathBuf,

    #[arg(long = "target", help = "Target source code name")]
    pub target: Option<String>,
}

pub struct ObfuscateCommandCore {
    dataset: Dataset,
    target: Option<String>,
}

impl ObfuscateCommandCore {
    pub fn new(args: ObfuscateCommand) -> Self {
        Self {
            dataset: Dataset::new(&args.dataset_json_path),
            target: args.target,
        }
    }
}

impl Command for ObfuscateCommandCore {
    fn run(&self, use_docker: bool) -> Result<()> {
        self.dataset
            .obfuscator_db
            .iter()
            .for_each(|obfuscator: &Obfuscator| {
                // 難読化のコマンドと共通パラメータを設定
                let (command, common_parameter): (&str, Vec<&str>) =
                    obfuscator.make_common_command(use_docker);

                self.dataset.code_db.iter().for_each(|code: &Code| {
                    if let Some(target) = &self.target {
                        if !code.dir_name.eq(target) {
                            return;
                        }
                    }

                    obfuscator
                        .transformation_set
                        .iter()
                        .for_each(|obfuscation: &Obfuscation| {
                            let mut args: Vec<&str> = vec![];

                            // 難読化手法ごとのパラメータ設定
                            let obfuscation_common_param: String =
                                obfuscation.get_parameter().replace(
                                    "--Functions=*",
                                    &format!("--Functions={}", code.function),
                                );
                            obfuscation_common_param
                                .split(' ')
                                .into_iter()
                                .map(|p| args.push(p))
                                .for_each(drop);

                            // 入出力に関するパラメータ設定
                            let code_path: PathBuf = self.dataset.src_dir.join(&code.target);
                            let dst_path: PathBuf = code_path.with_file_name(format!(
                                "obfuscated/{}.c",
                                obfuscation.display_name
                            ));
                            // 出力先が存在しない場合はディレクトリ作成
                            let dst_dir_path = dst_path.parent().unwrap();
                            if !dst_dir_path.exists() {
                                let _ = fs::create_dir(dst_dir_path);
                            }
                            let obfuscation_in_out_param: Vec<String> = vec![
                                format!(
                                    "-o {}",
                                    dst_path.with_extension("elf").to_string_lossy().to_string()
                                ),
                                code_path.to_string_lossy().to_string(),
                                format!("--out={}", dst_path.to_string_lossy()),
                            ];
                            obfuscation_in_out_param
                                .iter()
                                .map(|p| args.push(p))
                                .for_each(drop);

                            // コマンド実行
                            let parameter = [common_parameter.clone(), args].concat();
                            let target_command: Expression = cmd(command, parameter);
                            dbg!(&target_command);

                            // 成否判定(コードが生成されていれば，とりあえずOKとする)
                            let _output = target_command.unchecked().stderr_capture().run();
                        })
                });
            });

        Ok(())
    }
}
