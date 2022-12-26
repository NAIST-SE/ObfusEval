use anyhow::Result;
use console::style;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::builder::obfuscator::Transformation;
use crate::program::dataset::Dataset;

use crate::record::{code_distance::CodeDistance, code_test::CodeTest};

#[derive(Deserialize, Serialize, Debug)]
pub struct TransformationUnit {
    name: String,
    test_pass_rate: Option<CodeTestPassRate>,
    distance_mean: Option<CodeDistanceMean>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeTestPassRate {
    value: f32,
    exec_time_mean_increase_rate: f32,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct CodeDistanceMean {
    by_3gram_simpson: f32,
    by_longest_common_substring: f32,
    // Write here that your metrics.
}

impl TransformationUnit {
    pub fn new(t: &Transformation, dataset: &Dataset, step: &(bool, bool)) -> TransformationUnit {
        println!(
            "{} Measuring: {}({})",
            style("[+]").bold().dim(),
            t.name,
            t.short_name()
        );
        let file: PathBuf = PathBuf::from(&t.short_name()).with_extension("json");

        TransformationUnit {
            name: t.short_name(),
            test_pass_rate: match step.0 {
                true => {
                    println!("{} test pass rate...", style(" * ").bold().dim());

                    let record: Vec<CodeTest> = dataset
                        .code()
                        .par_iter()
                        .map(|code| {
                            let elf: PathBuf = code.elf(&dataset.elf_dir());
                            let obf_elf: PathBuf = code.obf_elf(&dataset.obf_t_dir(&t.name));
                            let test_file: PathBuf = code.test_file(&dataset.test_dir());
                            CodeTest::new(&elf, &obf_elf, &test_file)
                        })
                        .collect();
                    save_as_json(&record, dataset.result_test_dir().join(&file)).unwrap();

                    Some(CodeTestPassRate::new(&record))
                }
                false => None,
            },
            distance_mean: match step.1 {
                true => {
                    println!("{} code distance mean...", style(" * ").bold().dim());

                    let record: Vec<CodeDistance> = dataset
                        .code()
                        .par_iter()
                        .map(|code| {
                            let elf: PathBuf = code.elf(&dataset.elf_dir());
                            let obf_elf: PathBuf = code.obf_elf(&dataset.obf_t_dir(&t.name));
                            CodeDistance::new(&elf, &obf_elf)
                        })
                        .collect();
                    save_as_json(&record, dataset.result_distance_dir().join(&file)).unwrap();

                    Some(CodeDistanceMean::new(&record))
                }
                false => None,
            },
        }
    }

    pub fn output(record: &Vec<TransformationUnit>, dst: PathBuf) -> () {
        save_as_json(record, dst).unwrap()
    }
}

fn save_as_json<T: Serialize>(data: &Vec<T>, dst: PathBuf) -> Result<()> {
    let serialized: String = serde_json::to_string_pretty(&data).unwrap();

    let mut file = File::create(dst)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

impl CodeTestPassRate {
    pub fn new(record: &Vec<CodeTest>) -> CodeTestPassRate {
        let record_len: f32 = record.len() as f32;

        CodeTestPassRate {
            value: CodeTest::all_passed_test_code(record).len() as f32 / record_len,
            exec_time_mean_increase_rate: CodeTest::exec_time_mean_increase_rate(record),
        }
    }
}

impl CodeDistanceMean {
    pub fn new(record: &Vec<CodeDistance>) -> CodeDistanceMean {
        let record_len: f32 = record.len() as f32;

        CodeDistanceMean {
            by_3gram_simpson: record.iter().map(|r| r.by_3gram_simpson).sum::<f32>() / record_len,
            by_longest_common_substring: record
                .iter()
                .map(|r| r.by_longest_common_substring)
                .sum::<f32>() / record_len,
        }
    }
}
