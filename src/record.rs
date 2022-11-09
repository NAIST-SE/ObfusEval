use anyhow::Result;
use console::style;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::builder::Transformation;
use crate::evaluate::{code_distance_mean::SimilarityRecord, test_pass_rate::TestRecord};
use crate::program::{Code, Dataset};

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeUnitRecord {
    code_path: PathBuf,
    #[serde(flatten)]
    test: Option<TestRecord>,
    #[serde(flatten)]
    distance: Option<SimilarityRecord>,
}

impl CodeUnitRecord {
    fn new(
        code: &Code,
        t: &Transformation,
        dataset: &Dataset,
        step: &(bool, bool),
    ) -> CodeUnitRecord {
        let elf: PathBuf = code.elf(&dataset.elf_dir());
        let obf_elf: PathBuf = code.obf_elf(&dataset.obf_t_dir(&t.name));
        let test_file: PathBuf = code.test_file(&dataset.test_dir());

        let test_record: Option<TestRecord> = match step.0 {
            true => Some(TestRecord::new(&obf_elf, &test_file)),
            false => None,
        };

        let distance_record: Option<SimilarityRecord> = match step.1 {
            true => Some(SimilarityRecord::new(&elf, &obf_elf)),
            false => None,
        };

        CodeUnitRecord {
            code_path: obf_elf,
            test: test_record,
            distance: distance_record,
        }
    }

    fn test_records(records: &Vec<CodeUnitRecord>) -> Vec<(&PathBuf, &TestRecord)> {
        records
            .iter()
            .filter_map(|r| match r.test.as_ref() {
                Some(x) => Some((&r.code_path, x)),
                None => None,
            })
            .collect()
    }

    fn distance_records(records: &Vec<CodeUnitRecord>) -> Vec<(&PathBuf, &SimilarityRecord)> {
        records.iter().filter_map(|r| match r.distance.as_ref() {
            Some(x) => Some((&r.code_path, x)),
            None => None,
        }).collect()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransformationUnitRecord {
    transformation_name: String,
    test_pass_rate: Option<f32>,
    distance_mean: Option<Vec<(String, f32)>>,
}

impl TransformationUnitRecord {
    pub fn new(
        t: &Transformation,
        dataset: &Dataset,
        step: &(bool, bool),
    ) -> TransformationUnitRecord {
        println!(
            "{} Measuring: {}({})",
            style("[+]").bold().dim(),
            t.name,
            t.display
        );
        let file: PathBuf = PathBuf::from(&t.display).with_extension("json");

        let records: Vec<CodeUnitRecord> = dataset
            .code()
            .par_iter()
            .map(|code| CodeUnitRecord::new(code, t, dataset, step))
            .collect();

        if step.0 {
            println!("{} test pass rate...", style(" * ").bold().dim())
        };
        let test_pass_rate: Option<f32> = match step.0 {
            true => {
                let test_records: Vec<(&PathBuf, &TestRecord)> =
                    CodeUnitRecord::test_records(&records);
                save_as_json(&test_records, dataset.result_test_dir().join(&file)).unwrap();

                Some(TestRecord::calc_pass_rate(test_records))
            }
            false => None,
        };

        if step.1 {
            println!("{} code distance mean...", style(" * ").bold().dim())
        };
        let distance_mean: Option<Vec<(String, f32)>> = match step.1 {
            true => {
                let distance_records: Vec<(&PathBuf, &SimilarityRecord)> =
                    CodeUnitRecord::distance_records(&records);
                save_as_json(&distance_records, dataset.result_similarity_dir().join(&file)).unwrap();
                Some(SimilarityRecord::calc_distance_mean(distance_records))
            }
            false => None,
        };

        TransformationUnitRecord {
            transformation_name: t.display.clone(),
            test_pass_rate: test_pass_rate,
            distance_mean: distance_mean,
        }
    }

    pub fn output(records: &Vec<TransformationUnitRecord>, dst: PathBuf) -> () {
        save_as_json(records, dst).unwrap()
    }
}

fn save_as_json<T: Serialize>(data: &Vec<T>, dst: PathBuf) -> Result<()> {
    let serialized: String = serde_json::to_string_pretty(&data).unwrap();

    let mut file = File::create(dst)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
