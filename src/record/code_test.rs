use anyhow::{anyhow, Result};
use duct::cmd;
use quanta::{Clock, Instant};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Output;
use std::time::Duration;

use super::{
    super::program::{code::Code, testcase::Testcase},
    CodeTestResultMatchError,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeTest {
    name: String,
    test_results: Vec<CodeTestResult>,
    compare_results: Vec<CodeTestCompareResult>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeTestResult {
    args: String,
    execution_time: f32,
    exit_code: Option<i32>,
    stderr: String,
    stdout: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeTestCompareResult {
    fail_cause: Option<String>,
    exec_time_increase_rate: f32,
}

impl CodeTest {
    pub fn new(elf: &PathBuf, obfuscated_elf: &PathBuf, test_file: &PathBuf) -> CodeTest {
        let testcase: Vec<Testcase> = Testcase::new(test_file).expect("Can't open test_file");

        let test_results: Vec<CodeTestResult> =
            CodeTestResult::make(&elf, Testcase::all_args(&testcase));
        let obfuscated_test_results: Vec<CodeTestResult> =
            CodeTestResult::make(&obfuscated_elf, Testcase::all_args(&testcase));

        let compare_results = CodeTestCompareResult::make(&test_results, &obfuscated_test_results);

        CodeTest {
            name: elf.file_stem().unwrap().to_str().unwrap().to_string(),
            test_results: test_results,
            compare_results: compare_results,
        }
    }

    pub fn all_passed_test_code(record: &Vec<CodeTest>) -> Vec<&CodeTest> {
        record
            .into_iter()
            .filter(|r| r.compare_results.iter().all(|x| x.fail_cause.is_none()))
            .collect()
    }

    pub fn exec_time_mean_increase_rate(record: &Vec<CodeTest>) -> f32 {
        record
            .into_iter()
            .map(|r| {
                r.compare_results
                    .iter()
                    .fold(0.0, |sum, a| sum + a.exec_time_increase_rate)
                    / r.compare_results.len() as f32
            })
            .fold(0.0, |sum, x| sum + x)
            / record.len() as f32
    }
}

impl CodeTestResult {
    #[allow(unused_assignments)]
    pub fn new(elf: &PathBuf, args: &Vec<String>) -> CodeTestResult {
        let (output, exec_time): (Result<Output, std::io::Error>, Duration) =
            Code::run_with_exec_time(&elf, &args);

        let (exit_code, stderr, stdout): (Option<i32>, String, String) = match output {
            Ok(out) => (
                out.status.code(),
                String::from_utf8(out.stderr).unwrap(),
                String::from_utf8(out.stdout).unwrap(),
            ),
            Err(e) => (None, e.kind().to_string(), String::default()),
        };

        CodeTestResult {
            args: args.join(" "),
            execution_time: exec_time.as_secs_f32(),
            exit_code: exit_code,
            stderr: stderr,
            stdout: stdout,
        }
    }

    pub fn make(elf: &PathBuf, all_args: Vec<&Vec<String>>) -> Vec<CodeTestResult> {
        all_args
            .into_iter()
            .map(|args| CodeTestResult::new(&elf, args))
            .collect()
    }

    pub fn compare(&self, other: &CodeTestResult) -> Result<(), CodeTestResultMatchError> {
        if self.exit_code != other.exit_code {
            return Err(CodeTestResultMatchError::ExitCodeError(
                anyhow!(
                    "expect: \"{:?}\", actual: \"{:?}\"",
                    self.exit_code,
                    other.exit_code
                )
                .into(),
            ));
        } else if self.stdout != other.stdout {
            return Err(CodeTestResultMatchError::StdOutError(
                anyhow!("expect: \"{}\", actual: \"{}\"", self.stdout, other.stdout,).into(),
            ));
        } else if self.stderr != other.stderr {
            return Err(CodeTestResultMatchError::StdErrError(
                anyhow!("expect: \"{}\", actual: \"{}\"", self.stderr, other.stderr,).into(),
            ));
        }

        Ok(())
    }

    pub fn increase_exec_time_rate(&self, other: &CodeTestResult) -> f32 {
        other.execution_time / self.execution_time
    }
}

impl CodeTestCompareResult {
    pub fn new(original: &CodeTestResult, obfuscated: &CodeTestResult) -> CodeTestCompareResult {
        let fail_cause: Option<CodeTestResultMatchError> = original.compare(&obfuscated).err();

        CodeTestCompareResult {
            fail_cause: fail_cause.and_then(|x| Some(x.to_string())),
            exec_time_increase_rate: original.increase_exec_time_rate(obfuscated),
        }
    }

    pub fn make(
        test_results: &Vec<CodeTestResult>,
        obfuscated_test_results: &Vec<CodeTestResult>,
    ) -> Vec<CodeTestCompareResult> {
        test_results
            .iter()
            .zip(obfuscated_test_results.iter())
            .map(|(x, y)| CodeTestCompareResult::new(x, y))
            .collect()
    }
}

impl Code {
    fn run(elf: &PathBuf, args: &Vec<String>) -> Result<Output, std::io::Error> {
        cmd(elf, args)
            .unchecked()
            .stdout_capture()
            .stderr_capture()
            .run()
    }

    #[allow(unused_assignments)]
    fn run_with_exec_time(
        elf: &PathBuf,
        args: &Vec<String>,
    ) -> (Result<Output, std::io::Error>, Duration) {
        let clock: Clock = Clock::new();
        let start: Instant = clock.now();
        let mut stop: Instant = start;

        let output: Result<Output, std::io::Error> = Code::run(&elf, &args);

        stop = clock.now();
        let exec_time: Duration = stop.duration_since(start);

        (output, exec_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_test_record() {
        let elf: PathBuf = PathBuf::from("tests/sample-output/original-elf/binarysearch.elf")
            .canonicalize()
            .unwrap();
        let obfuscated_elf: PathBuf =
            PathBuf::from("tests/sample-output/original-elf/binarysearch.elf")
                .canonicalize()
                .unwrap();
        let test_file: PathBuf = PathBuf::from("tests/sample-dataset/test/binarysearch.json");

        dbg!(CodeTest::new(&elf, &obfuscated_elf, &test_file));
    }

    #[test]
    fn code_test_result() {
        let elf: PathBuf = PathBuf::from("tests/sample-output/original-elf/mergesort.elf")
            .canonicalize()
            .unwrap();
        let test_file: PathBuf = PathBuf::from("tests/sample-dataset/test/mergesort.json");
        let testcase: Vec<Testcase> = Testcase::new(&test_file).expect("Can't open test_file");
        let all_args: Vec<&Vec<String>> = Testcase::all_args(&testcase);

        dbg!(CodeTestResult::make(&elf, all_args));
    }

    #[test]
    fn code_test_compare_result() {}

    #[test]
    fn code_run() {
        // let elf: PathBuf = PathBuf::from("tests/sample-output/obfuscated-code/encodeArithmetic1_addOpaque16/mergesort.elf").canonicalize().unwrap();
        // let elf: PathBuf = PathBuf::from("tests/sample-output/original-elf/seg_fault.elf").canonicalize().unwrap();
        // let test_file: PathBuf = PathBuf::from("tests/sample-dataset/test/seg_fault.json");
        let elf: PathBuf = PathBuf::from("tests/sample-output/original-elf/mergesort.elf")
            .canonicalize()
            .unwrap();
        let test_file: PathBuf = PathBuf::from("tests/sample-dataset/test/mergesort.json");
        let testcase: Vec<Testcase> = Testcase::new(&test_file).expect("Can't open test_file");
        let all_args: Vec<&Vec<String>> = Testcase::all_args(&testcase);

        let res0 = Code::run(&elf, &all_args.get(0).unwrap());
        dbg!(res0.is_err());
        // dbg!(Code::run(&elf, &all_args.get(1).unwrap()));
    }
}
