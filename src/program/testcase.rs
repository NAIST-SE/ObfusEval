use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Testcase {
    args: Vec<String>,
    stdout: String,
    stderr: String,
    exit_status: i32,
}

impl Testcase {
    pub fn new(test_file: &PathBuf) -> Result<Vec<Testcase>> {
        let file: File = File::open(test_file)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn all_args(cases: &Vec<Testcase>) -> Vec<&Vec<String>> {
        cases.iter().map(|case| &case.args).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_func() {
        let test_file: PathBuf = PathBuf::from("tests/sample-dataset/test/mergesort.json");
        let testcase: Result<Vec<Testcase>, anyhow::Error> = Testcase::new(&test_file);

        match testcase {
            Ok(t) => {
                assert_eq!(t, vec![Testcase {
                    args: vec![],
                    stdout: "".to_string(),
                    stderr: "".to_string(),
                    exit_status: 1,
                },
                Testcase {
                    args: vec![
                        "10".to_string(),
                        "130".to_string(),
                        "13215".to_string(),
                        "136".to_string(),
                        "31".to_string(),
                        "34".to_string(),
                        "39".to_string(),
                        "57".to_string(),
                        "59".to_string(),
                        "66".to_string(),
                    ],
                    stdout: "After merge sorting elements are: 10 130 13215 136 31 34 39 57 59 66 ".to_string(),
                    stderr: "".to_string(),
                    exit_status: 0,
                },
                Testcase {
                    args: vec![
                        "36".to_string(),
                        "15".to_string(),
                        "79".to_string(),
                        "48".to_string(),
                        "46".to_string(),
                        "36".to_string(),
                        "37".to_string(),
                        "18".to_string(),
                        "28".to_string(),
                        "27".to_string(),
                    ],
                    stdout: "After merge sorting elements are: 36 15 79 48 46 36 37 18 28 27 ".to_string(),
                    stderr: "".to_string(),
                    exit_status: 0,
                }]);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn all_args() {
        let test_file: PathBuf = PathBuf::from("tests/sample-dataset/test/mergesort.json");
        let testcase: Vec<Testcase> = Testcase::new(&test_file).unwrap();

        assert_eq!(
            Testcase::all_args(&testcase),
            vec![
                &vec![],
                &vec![
                    "10".to_string(),
                    "130".to_string(),
                    "13215".to_string(),
                    "136".to_string(),
                    "31".to_string(),
                    "34".to_string(),
                    "39".to_string(),
                    "57".to_string(),
                    "59".to_string(),
                    "66".to_string(),
                ],
                &vec![
                    "36".to_string(),
                    "15".to_string(),
                    "79".to_string(),
                    "48".to_string(),
                    "46".to_string(),
                    "36".to_string(),
                    "37".to_string(),
                    "18".to_string(),
                    "28".to_string(),
                    "27".to_string(),
                ]
            ]
        );
    }
}
