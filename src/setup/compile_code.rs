use anyhow::Result;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process;
use thiserror::Error;

use crate::builder::service::DockerService;
use crate::program::{code::Code, dataset::Dataset};

#[derive(Debug, Error)]
pub enum SetupError {
    #[error("CompileCodeError: {0}\n{1}")]
    CompileCodeError(PathBuf, anyhow::Error),
}

impl Code {
    pub fn check_libm(src: &PathBuf) -> Result<bool> {
        let content: String = read_to_string(src)?;
        let res: Vec<&str> = content
            .lines()
            .filter(|line| line.contains("#include <math.h>"))
            .collect();
        Ok(!res.is_empty())
    }

    fn compile(src: PathBuf, elf: PathBuf, compiler: &DockerService) -> Result<()> {
        if elf.exists() {
            return Ok(());
        }

        let mut args = vec!["-o", &elf.to_str().unwrap(), &src.to_str().unwrap()];
        if Code::check_libm(&src)? {
            args.push("-lm");
        }

        if let Err(e) = compiler.run(args) {
            return Err(SetupError::CompileCodeError(src, e).into());
        }

        Ok(())
    }
}

impl Dataset {
    pub fn make_bin(&self, compiler: &DockerService) -> Result<()> {
        let code = self.code();

        let fail: Vec<_> = code
            .par_iter()
            .progress_count((&code).len() as u64)
            .filter_map(|c| {
                Code::compile(c.src(&self.src_dir()), c.elf(&self.elf_dir()), compiler).err()
            })
            .collect();

        if !fail.is_empty() {
            for e in fail {
                eprintln!("{}", e);
            }
            process::exit(1);
        }

        Ok(())
    }
}
