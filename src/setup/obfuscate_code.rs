use anyhow::Result;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use thiserror::Error;
use std::path::PathBuf;

use crate::builder::obfuscator::{Obfuscator, Transformation};
use crate::program::{code::Code, dataset::Dataset};

#[derive(Debug, Error)]
pub enum SetupError {
    #[error("ObfuscateCodeError: {0}\n{1}")]
    ObfuscateCodeError(PathBuf, anyhow::Error),
    #[error("ObfuscateError: \n{:?}", .0)]
    ObfuscateCodesError(Vec<anyhow::Error>),
}

impl Code {
    fn obfuscate(
        src: PathBuf,
        obf_src: PathBuf,
        elf: PathBuf,
        obfuscator: &Obfuscator,
        t: &Transformation,
    ) -> Result<()> {
        if elf.exists() {
            return Ok(());
        }

        let flag: String = t.flag();
        let mut args: Vec<&str> = vec![flag.as_str()];
        let out_option: String = format!("--out={}", obf_src.to_str().unwrap());
        if !obfuscator.is_bin_only {
            args.push(out_option.as_str());
        }
        vec!["-o", &elf.to_str().unwrap(), &src.to_str().unwrap()]
            .iter()
            .map(|o| args.push(o))
            .for_each(drop);
        if Code::check_libm(&src)? {
            args.push("-lm");
        }

        if let Err(e) = obfuscator.run(args) {
            return Err(SetupError::ObfuscateCodeError(src, e).into());
        }

        Ok(())
    }
}

impl Dataset {
    pub fn make_obf_code(&self, obfuscator: &Obfuscator, t: &Transformation) -> Result<()> {
        let code = self.code();

        let fail: Vec<_> = code
            .par_iter()
            .progress_count((&code).len() as u64)
            .filter_map(|c| {
                    Code::obfuscate(
                        c.src(&self.src_dir()),
                        c.obf_src(&self.obf_t_dir(&t.name)),
                        c.obf_elf(&self.obf_t_dir(&t.name)),
                        obfuscator,
                        t,
                    ).err()
            })
            .collect();
        
        if !fail.is_empty() {
            return Err(SetupError::ObfuscateCodesError(fail).into());
        }

        Ok(())
    }
}
