use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::create_dir;
use std::path::PathBuf;

use super::code::Code;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
// Dataset instance have Code instances.
pub struct Dataset {
    data_dir: PathBuf,
    working_dir: PathBuf,
}

impl Dataset {
    pub fn new(p: PathBuf, name: String) -> Result<Dataset> {
        let working_dir: PathBuf = PathBuf::from("output").join(&name);
        mkdir(&working_dir)?;

        Ok(Dataset {
            data_dir: p.canonicalize()?,
            working_dir: working_dir.canonicalize()?,
        })
    }

    pub fn init(&self, transformations_name: &Vec<&String>) -> Result<()> {
        mkdir(&self.elf_dir())?;
        mkdir(&self.obf_dir())?;
        mkdir(&self.result_test_dir())?;
        mkdir(&self.result_distance_dir())?;

        transformations_name
            .iter()
            .inspect(|name| {
                mkdir(&self.obf_dir().join(name)).ok();
            })
            .for_each(drop);
        Ok(())
    }
}

// Implementation for getting path
impl Dataset {
    pub fn src_dir(&self) -> PathBuf {
        self.data_dir.join("src")
    }

    pub fn test_dir(&self) -> PathBuf {
        self.data_dir.join("test")
    }

    pub fn result_dir(&self) -> &PathBuf {
        &self.working_dir
    }

    pub fn result_test_dir(&self) -> PathBuf {
        self.working_dir.join("result_test")
    }

    pub fn result_distance_dir(&self) -> PathBuf {
        self.working_dir.join("result_distance")
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
}

// Implementaiton for returning Code instances
impl Dataset {
    pub fn code(&self) -> Vec<Code> {
        self.src_dir()
            .read_dir()
            .unwrap()
            .into_iter()
            .filter_map(|x| x.ok())
            .map(|y| Code::new(&y.path().file_stem().unwrap().to_str().unwrap()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;

    #[test]
    fn new_and_init_func() {
        let p: PathBuf = PathBuf::from("tests/sample-dataset");
        let name: String = "tests/sample-output-temp".to_string();
        let dataset = Dataset::new(p, name);
        let trans1: String = "sample1".to_string();
        let transformations_name: Vec<&String> = vec![&trans1];

        match dataset {
            Ok(d) => {
                match d.init(&transformations_name) {
                    Ok(_) => {
                        assert!(remove_dir_all(&d.working_dir).is_ok());
                    }
                    Err(_) => {
                        assert!(false, "error in Dataset::init()");
                    }
                };
            }
            Err(_) => {
                assert!(false, "error in Dataset::new()");
            }
        }
    }

    #[test]
    fn each_path_get_func() {
        let p: PathBuf = PathBuf::from("tests/sample-dataset");
        let name: String = "tests/sample-output-temp".to_string();
        let dataset = Dataset::new(p, name).unwrap();
        let trans1: String = "sample1".to_string();
        let transformations_name: Vec<&String> = vec![&trans1];

        dataset.init(&transformations_name).unwrap();

        assert_eq!(
            dataset.src_dir(),
            PathBuf::from("tests/sample-dataset/src")
                .canonicalize()
                .unwrap()
        );

        assert_eq!(
            dataset.test_dir(),
            PathBuf::from("tests/sample-dataset/test")
                .canonicalize()
                .unwrap()
        );

        assert_eq!(
            dataset.result_dir(),
            &PathBuf::from("tests/sample-output-temp")
                .canonicalize()
                .unwrap()
        );

        assert_eq!(
            dataset.result_test_dir(),
            PathBuf::from("tests/sample-output-temp/result_test")
                .canonicalize()
                .unwrap()
        );

        assert_eq!(
            dataset.result_distance_dir(),
            PathBuf::from("tests/sample-output-temp/result_distance")
                .canonicalize()
                .unwrap()
        );

        assert_eq!(
            dataset.elf_dir(),
            PathBuf::from("tests/sample-output-temp/original-elf")
                .canonicalize()
                .unwrap()
        );

        assert_eq!(
            dataset.obf_dir(),
            PathBuf::from("tests/sample-output-temp/obfuscated-code")
                .canonicalize()
                .unwrap()
        );

        assert_eq!(
            dataset.obf_t_dir(&trans1),
            PathBuf::from("tests/sample-output-temp/obfuscated-code/sample1")
                .canonicalize()
                .unwrap()
        );

        remove_dir_all(&dataset.working_dir).unwrap();
    }

    #[test]
    fn code_instances() {
        let p: PathBuf = PathBuf::from("tests/sample-dataset");
        let name: String = "tests/sample-output-temp".to_string();
        let dataset = Dataset::new(p, name).unwrap();

        assert_eq!(dataset.code(), vec![Code::new("mergesort")]);
        remove_dir_all(&dataset.working_dir).unwrap();
    }
}
