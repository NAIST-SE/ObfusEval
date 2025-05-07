use crate::code_processor::{Executor, ExecutorInfo};

use super::{Compiler, CompilerOption, CompilerSerializeModel};

#[derive(Debug)]
pub struct Gcc {
    executor: ExecutorInfo,
    _target_language: String,
    _target_arch: String,
    flags: Vec<String>,
    coverage_flags: Vec<String>,
    test_flags: Vec<String>,
}

impl Executor for Gcc {
    fn label(&self) -> &str {
        &self.executor.label
    }
    fn name(&self) -> &str {
        &self.executor.name
    }
    fn version(&self) -> &str {
        &self.executor.version
    }
    fn bin_path(&self) -> &str {
        &self.executor.binary_path
    }
    fn bin_args(&self) -> Option<Vec<&str>> {
        match &self.executor.binary_args {
            Some(bin_args) => Some(bin_args.iter().map(String::as_str).collect()),
            _ => None,
        }
    }
}

impl Compiler for Gcc {
    fn new(executor: ExecutorInfo, value: CompilerSerializeModel) -> Self
    where
        Self: Sized,
    {
        Self {
            executor: executor,
            _target_language: value.target_language,
            _target_arch: value.target_arch,
            flags: value.flags,
            coverage_flags: value.coverage_flags,
            test_flags: value.test_flags,
        }
    }

    fn flags(&self, option: &CompilerOption) -> Vec<String> {
        let mut flags: Vec<String> = Vec::new();
        flags.push(option.optimization_flag());
        if *option.enable_coverage() {
            flags.extend(self.coverage_flags.clone());
        }
        if *option.enable_test() {
            flags.extend(self.test_flags.clone());
        }
        flags.extend(self.flags.clone());
        flags
    }
}
