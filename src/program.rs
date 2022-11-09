use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::create_dir;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
pub struct Code {
    name: String,
}

impl Code {
    pub fn src(&self, src_dir: &PathBuf) -> PathBuf {
        src_dir.join(&self.name).with_extension("c")
    }

    pub fn elf(&self, elf_dir: &PathBuf) -> PathBuf {
        elf_dir.join(&self.name).with_extension("elf")
    }

    pub fn obf_src(&self, obf_src_dir: &PathBuf) -> PathBuf {
        obf_src_dir.join(&self.name).with_extension("c")
    }

    pub fn obf_elf(&self, obf_elf_dir: &PathBuf) -> PathBuf {
        obf_elf_dir.join(&self.name).with_extension("elf")
    }

    pub fn test_file(&self, test_dir: &PathBuf) -> PathBuf {
        test_dir.join(&self.name).with_extension("json")
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Dataset {
    data_dir: PathBuf,
    working_dir: PathBuf,
}

impl Dataset {
    pub fn new(p: PathBuf, name: String) -> Result<Dataset> {
        Ok(Dataset {
            data_dir: p.canonicalize()?,
            working_dir: PathBuf::from(format!("output/{}", &name)),
        })
    }

    pub fn init(&self, transformations_name: &Vec<&String>) -> Result<()> {
        mkdir(&self.working_dir)?;
        mkdir(&self.elf_dir())?;
        mkdir(&self.obf_dir())?;
        mkdir(&self.result_test_dir())?;
        mkdir(&self.result_similarity_dir())?;

        transformations_name
            .iter()
            .inspect(|name| {
                mkdir(&self.obf_dir().join(name)).ok();
            })
            .for_each(drop);
        Ok(())
    }
    
    pub fn result_test_dir(&self) -> PathBuf {
        self.working_dir.join("result_test")
    }

    pub fn result_similarity_dir(&self) -> PathBuf {
        self.working_dir.join("result_similarity")
    }

    pub fn result_dir(&self) -> &PathBuf {
        &self.working_dir
    }

    pub fn src_dir(&self) -> PathBuf {
        self.data_dir.join("src")
    }

    pub fn test_dir(&self) -> PathBuf {
        self.data_dir.join("test")
    }

    pub fn elf_dir(&self) -> PathBuf {
        self.working_dir.join("original-elf")
    }

    pub fn obf_dir(&self) -> PathBuf {
        self.working_dir.join("obfuscated-code")
    }

    pub fn obf_t_dir(&self, t: &String) -> PathBuf {
        self.obf_dir().join(t)
    }

    pub fn code(&self) -> Vec<Code> {
        self.src_dir()
            .read_dir()
            .unwrap()
            .into_iter()
            .filter_map(|x| x.ok())
            .map(|y| Code {
                name: y.file_name().to_str().unwrap().to_string(),
            })
            .collect()
    }
}

fn mkdir(target: &PathBuf) -> Result<()> {
    if let Err(_) = target.canonicalize() {
        if !target.exists() {
            create_dir(&target)?;
        }
        target.canonicalize()?;
    }
    Ok(())
}
