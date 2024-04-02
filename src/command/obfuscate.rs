use anyhow::Result;
use duct::{cmd, Expression};
use std::{fs, path::PathBuf};

use clap::Parser;

use crate::model::{code::Code, obfuscation::Obfuscation, obfuscator::Obfuscator};

use super::Command;

#[derive(Parser)]
pub struct ObfuscateCommand {
    #[arg(
        long = "obfuscator-db",
        help = "Path to the json file that manages Obfuscator information"
    )]
    pub obfuscator_db_path: PathBuf,

    #[arg(
        long = "code-db",
        help = "Path to the json file that manages code name each code in dataset"
    )]
    pub code_db_path: PathBuf,

    #[arg(help = "Target source code name")]
    pub target: String,
}

pub struct ObfuscateCommandCore {
    obfuscator_db: Obfuscator,
    code_db: Vec<Code>,
    src: PathBuf,
    target: String,
}

impl ObfuscateCommandCore {
    pub fn new(args: ObfuscateCommand) -> Self {
        Self {
            obfuscator_db: Obfuscator::new(&args.obfuscator_db_path),
            code_db: Code::new(&args.code_db_path),
            src: fs::canonicalize(&args.code_db_path.parent().unwrap()).unwrap(),
            target: args.target,
        }
    }
}

impl Command for ObfuscateCommandCore {
    fn run(&self, use_docker: bool) -> Result<()> {
        // 難読化のコマンドと共通パラメータを設定
        let (command, common_parameter): (&str, Vec<&str>) =
            self.obfuscator_db.make_common_command(use_docker);

        // Todo: すべてのコードに対して難読化を実行するオプションを追加する．
        self.code_db.iter().for_each(|code: &Code| {
            if !code.name.eq(&self.target) {
                return;
            }

            self.obfuscator_db
                .transformation_set
                .iter()
                .for_each(|obfuscation: &Obfuscation| {
                    let mut args: Vec<&str> = vec![];

                    // 難読化手法ごとのパラメータ設定
                    let obfuscation_common_param: String = obfuscation
                        .get_parameter()
                        .replace("--Functions=*", &format!("--Functions={}", code.function));
                    obfuscation_common_param
                        .split(' ')
                        .into_iter()
                        .map(|p| args.push(p))
                        .for_each(drop);

                    // 入出力に関するパラメータ設定
                    let code_path: PathBuf = self.src.join(&code.path);
                    let dst_path: PathBuf = code_path
                        .with_file_name(format!("obfuscated/{}", obfuscation.display_name));
                    // 出力先が存在しない場合はディレクトリ作成
                    let dst_dir_path = dst_path.parent().unwrap();
                    if !dst_dir_path.exists() {
                        let _ = fs::create_dir(dst_dir_path);
                    }
                    let obfuscation_in_out_param: Vec<String> = vec![
                        format!("-o {}", dst_path.with_extension("elf").to_string_lossy()),
                        code_path.to_string_lossy().to_string(),
                        format!("--out={}", dst_path.to_string_lossy().to_string()),
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

        Ok(())
    }
}
