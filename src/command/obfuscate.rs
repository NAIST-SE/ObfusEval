use anyhow::Result;
use duct::{cmd, Expression};
use std::{fs, path::PathBuf};

use clap::Parser;

use crate::model::{
    code::CodeInfo, dataset::Dataset, obfuscation::Obfuscation, obfuscator::Obfuscator,
};

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
        self.dataset
            .obfuscator_db
            .iter()
            .for_each(|obfuscator: &Obfuscator| {
                // 難読化のコマンドと共通パラメータを設定
                let (command, common_parameter): (&str, Vec<&str>) =
                    obfuscator.make_common_command(self.use_docker_compose);

                self.dataset.code_db.iter().for_each(|code: &CodeInfo| {
                    if let Some(target) = &self.target {
                        if !code.dir_name.eq(target) {
                            return;
                        }
                    }

                    let src_path: PathBuf = code.get_src_path(&self.dataset.src_dir);
                    let dst_dir_path: PathBuf = code.get_dst_dir_path(&self.dataset.src_dir);
                    // 出力先が存在しない場合はディレクトリ作成
                    if !dst_dir_path.exists() {
                        let _ = fs::create_dir(&dst_dir_path);
                    }

                    obfuscator
                        .transformation_set
                        .iter()
                        .for_each(|obfuscation: &Obfuscation| {
                            let dst_path: &PathBuf = &dst_dir_path
                                .join(&obfuscation.display_name)
                                .with_extension("c");
                            // dbg!(&dst_path);
                            if dst_path.exists() {
                                return;
                            }

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
                            let obfuscation_in_out_param: Vec<String> = vec![
                                format!(
                                    "-o {}",
                                    dst_path.with_extension("elf").to_string_lossy().to_string()
                                ),
                                src_path.to_string_lossy().to_string(),
                                format!("--out={}", dst_path.to_string_lossy()),
                            ];
                            obfuscation_in_out_param
                                .iter()
                                .map(|p| args.push(p))
                                .for_each(drop);

                            // コマンド実行
                            let parameter = [common_parameter.clone(), args].concat();
                            let target_command: Expression = cmd(command, parameter);
                            // dbg!(&target_command);

                            // 成否判定(コードが生成されていれば，とりあえずOKとする)
                            let _output = target_command.unchecked().stderr_capture().run();
                        })
                });
            });

        Ok(())
    }
}
