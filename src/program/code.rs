use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Code {
    name: String,
}

impl Code {
    pub fn new(target: &str) -> Code {
        Code { name: target.to_string() }
    }
}

// Implementation for getting path
impl Code {
    pub fn src(&self, src_dir: &PathBuf) -> PathBuf {
        src_dir.join(&self.name).with_extension("c")
    }

    pub fn test_file(&self, test_dir: &PathBuf) -> PathBuf {
        test_dir.join(&self.name).with_extension("json")
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_func() {
        let code = Code::new("mergesort");
        assert_eq!(code, Code { name: "mergesort".to_string() });
    }

    #[test]
    fn each_path_get_func() {
        let code = Code::new("mergesort");

        let src_dir: PathBuf = PathBuf::from("tests/sample-dataset/src");
        assert_eq!(code.src(&src_dir), PathBuf::from("tests/sample-dataset/src/mergesort.c"));

        let test_dir: PathBuf = PathBuf::from("tests/sample-dataset/test");
        assert_eq!(code.test_file(&test_dir), PathBuf::from("tests/sample-dataset/test/mergesort.json"));

        let elf_dir: PathBuf = PathBuf::from("tests/sample-output/original-elf");
        assert_eq!(code.elf(&elf_dir), PathBuf::from("tests/sample-output/original-elf/mergesort.elf"));

        let obf_dir: PathBuf = PathBuf::from("tests/sample-output/obfuscated-code/encodeArithmetic1_addOpaque16");
        assert_eq!(code.obf_src(&obf_dir), PathBuf::from("tests/sample-output/obfuscated-code/encodeArithmetic1_addOpaque16/mergesort.c"));
        assert_eq!(code.obf_elf(&obf_dir), PathBuf::from("tests/sample-output/obfuscated-code/encodeArithmetic1_addOpaque16/mergesort.elf"));
    }
}
