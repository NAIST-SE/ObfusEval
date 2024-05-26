use std::{io::Write, path::PathBuf};

use duct::{cmd, Expression};

use indicatif::{ProgressBar, ProgressStyle};
use obfus_eval::model::dataset::Dataset;

#[test]
fn compile_test() {
    // 難読化されたコードを調整後，コンパイルを行いコンパイルが通るかどうかをテストする
    let target_db: PathBuf = PathBuf::from("tests/resource/algo-benchmarks/dataset.json");
    let dataset = Dataset::new(&target_db);

    for code in dataset.code_db.iter() {
        let dst_adj_dir_path = code.get_dst_adj_dir_path(&dataset.src_dir);
        if !dst_adj_dir_path.exists() {
            continue;
        }

        let test_cpp = PathBuf::from(format!("src/{}/test.cpp", code.dir_name));
        let test_common_cpp = PathBuf::from("src/test_common.cpp");

        if !code.get_dst_adj_dir_path(&dataset.src_dir).exists() {
            continue;
        }

        let adjusted_code = code
            .get_dst_adj_dir_path(&dataset.src_dir)
            .read_dir()
            .unwrap()
            .into_iter()
            .filter_map(|result| result.ok());
        let bar = ProgressBar::new(adjusted_code.count() as u64);
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap(),
        );

        code.get_dst_adj_dir_path(&dataset.src_dir)
            .read_dir()
            .unwrap()
            .into_iter()
            .filter_map(|result| result.ok())
            .for_each(|result| {
                if !&result.path().extension().unwrap().eq("c") {
                    return;
                }

                let target: PathBuf = result.path();

                // コンパイルの実行
                let common_args: Vec<String> = vec![
                    "-Wall",
                    "-fprofile-arcs",
                    "-ftest-coverage",
                    "-lgtest",
                    "-lgtest_main",
                    "-pthread",
                    "-o",
                    "test.elf",
                ]
                .into_iter()
                .map(|s| s.to_string())
                .collect();
                let io_args: Vec<String> = vec![test_cpp.clone(), test_common_cpp.clone(), target]
                    .into_iter()
                    .map(|p| p.into_os_string().into_string().unwrap())
                    .collect();

                let args: Vec<String> = [io_args, common_args].concat();

                let command: Expression = cmd("g++", args);
                let output = command.stderr_capture().run();

                if output.is_err() {
                    writeln!(
                        &mut std::io::stderr(),
                        "{}",
                        String::from_utf8(
                            command.unchecked().stderr_capture().run().unwrap().stderr
                        )
                        .unwrap()
                    )
                    .unwrap();
                }

                assert!(output.is_ok());

                bar.inc(1);
                bar.set_message(format!(
                    "{}::{}",
                    code.target.trim_end_matches(".c"),
                    result
                        .path()
                        .file_name()
                        .unwrap()
                        .to_os_string()
                        .into_string()
                        .unwrap()
                        .trim_end_matches(".c")
                ));
            });
    }
}
