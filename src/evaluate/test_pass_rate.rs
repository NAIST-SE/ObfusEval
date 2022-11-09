use anyhow::{anyhow, Result};
use duct::cmd;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use std::process::Output;

use super::EvaluateError;
use crate::program::Code;

#[derive(Deserialize, Serialize, Debug)]
pub struct TestRecord {
    args: Vec<Vec<String>>,
    fail_cause: Vec<Option<String>>,
}

impl TestRecord {
    pub fn new(elf: &PathBuf, test_file: &PathBuf) -> TestRecord {
        let testcases = Testcase::new(test_file).expect("Can't open test_file");

        let all_args: Vec<Vec<String>> = testcases
            .iter()
            .map(|testcase| testcase.args.clone())
            .collect();

        let res: Vec<Option<String>> = testcases
            .iter()
            .map(|testcase| Code::test(&elf, testcase).map_err(|e| e.to_string()).err())
            .collect();

        TestRecord {
            args: all_args,
            fail_cause: res,
        }
    }

    pub fn calc_pass_rate(records: Vec<(&PathBuf, &TestRecord)>) -> f32 {
        let records_len: f32 = records.len() as f32;

        let pass: Vec<(&PathBuf, &TestRecord)> = records
            .into_iter()
            .filter(|(_, r)| r.fail_cause.iter().all(|x| x.is_none()))
            .collect();

        pass.len() as f32 / records_len
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Testcase {
    args: Vec<String>,
    stdout: String,
    stderr: String,
    exit_status: Option<i32>,
}

impl Testcase {
    fn new(test_file: &PathBuf) -> Result<Vec<Testcase>> {
        let file: File = File::open(test_file)?;
        Ok(serde_json::from_reader(file)?)
    }

    fn compare(&self, output: std::process::Output) -> Result<()> {
        if output.status.code() != self.exit_status {
            return Err(EvaluateError::FailTestError(anyhow!("exit_code do not match")).into())
        }

        if String::from_utf8(output.stdout)? != self.stdout {
            return Err(EvaluateError::FailTestError(anyhow!("stdout do not match")).into())
        }

        if String::from_utf8(output.stderr)? != self.stderr {
            return Err(EvaluateError::FailTestError(anyhow!("stderr do not match")).into())
        }

        Ok(())
    }
}

impl Code {
    fn test(elf: &PathBuf, testcase: &Testcase) -> Result<()> {
        match Code::run(&elf, &testcase.args) {
            Ok(r) => match testcase.compare(r) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            Err(e) => Err(EvaluateError::RucCodeError(anyhow!(e)).into()),
        }
    }

    fn run(elf: &PathBuf, args: &Vec<String>) -> Result<Output, std::io::Error> {
        cmd(elf, args)
            .unchecked()
            .stdout_capture()
            .stderr_capture()
            .run()
    }
}
