use duct::cmd;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

use super::super::program::code::Code;


#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct CodeDistance {
    name: String,
    pub by_3gram_simpson: f32,
    // Write here that your metrics.
}


impl CodeDistance {
    pub fn new(elf_a: &PathBuf, elf_b: &PathBuf) -> CodeDistance {
        CodeDistance {
            name: elf_a.file_stem().unwrap().to_str().unwrap().to_string(),
            by_3gram_simpson: CodeDistance::trigram_simpson(&elf_a, &elf_b),
        }
    }
}

impl CodeDistance {
    fn trigram_simpson(elf_a: &PathBuf, elf_b: &PathBuf) -> f32 {
        let n = 3;

        let a: HashSet<Vec<String>> = HashSet::from_iter(Code::n_gram_opcode(elf_a, n));
        let b: HashSet<Vec<String>> = HashSet::from_iter(Code::n_gram_opcode(elf_b, n));

        let numerator: f32 = a.intersection(&b).collect::<Vec<_>>().len() as f32;
        let denominator: f32 = if a.len() < b.len() { a.len() } else { b.len() } as f32;

        1.0 - numerator / denominator
    }
}

impl Code {
    fn n_gram_opcode(elf: &PathBuf, n: usize) -> Vec<Vec<String>> {
        Code::opcode(elf)
            .windows(n)
            .into_iter()
            .map(|x| x.to_vec())
            .collect()
    }

    fn opcode(elf: &PathBuf) -> Vec<String> {
        let args = [
            "-d",
            "-j",
            ".text",
            "--prefix-address",
            elf.to_str().unwrap(),
        ];
        let output = cmd("objdump", &args).read().expect("objdump error");

        output
            .lines()
            .skip(5)
            .filter_map(|l| l.split_whitespace().nth(2))
            .map(|l| l.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn code_distance_record() {
        let elf_a: PathBuf = PathBuf::from("tests/sample-output/original-elf/mergesort.elf");
        let elf_b: PathBuf = PathBuf::from(
            "tests/sample-output/obfuscated-code/encodeArithmetic1_addOpaque16/mergesort.elf",
        );

        assert_eq!(
            CodeDistance::new(&elf_a, &elf_b),
            CodeDistance {
                name: "mergesort".to_string(),
                by_3gram_simpson: 0.26993865,
            }
        );
    }

    // #[test]
    // fn code_distance_mean() {
    //     let elf_a: PathBuf = PathBuf::from("tests/sample-output/original-elf/mergesort.elf");
    //     let elf_b: PathBuf = PathBuf::from(
    //         "tests/sample-output/obfuscated-code/encodeArithmetic1_addOpaque16/mergesort.elf",
    //     );

    //     let res: CodeDistance = CodeDistance::new(&elf_a, &elf_b);
    //     let record: Vec<&CodeDistance> = vec![&res];

    //     assert_eq!(
    //         CodeDistanceMean::new("EncA_addOp16".to_string(), record),
    //         CodeDistanceMean {
    //             name: "EncA_addOp16".to_string(),
    //             by_3gram_simpson: 0.26993865,
    //             by_longest_common_substring: 0.95519894,
    //         }
    //     );
    // }

    #[test]
    fn trigram_simpson() {
        let elf_a: PathBuf = PathBuf::from("tests/sample-output/original-elf/mergesort.elf");
        let elf_b: PathBuf = PathBuf::from(
            "tests/sample-output/obfuscated-code/encodeArithmetic1_addOpaque16/mergesort.elf",
        );

        assert_eq!(CodeDistance::trigram_simpson(&elf_a, &elf_b), 0.26993865);
    }

    #[test]
    fn code_opcode() {
        let elf: PathBuf = PathBuf::from("tests/sample-output/original-elf/mergesort.elf");

        let file: PathBuf = PathBuf::from("tests/sample-output/original-opcode.txt");
        let mut f: File = File::open(file).expect("file not found");
        let mut contents: String = String::new();
        f.read_to_string(&mut contents).expect("something went wrong reading the file");
        let opcode: Vec<String> = contents.split_whitespace().into_iter().map(|x| x.to_string()).collect();
        
        assert_eq!(Code::opcode(&elf), opcode);
    }

    #[test]
    fn code_n_gram_opcode() {
        let elf: PathBuf = PathBuf::from("tests/sample-output/original-elf/mergesort.elf");

        let trigram_opcode: Vec<Vec<String>> = Code::n_gram_opcode(&elf, 3);
        assert_eq!(trigram_opcode.get(0).unwrap(), &vec![
            "xor".to_string(),
            "mov".to_string(),
            "pop".to_string(),
        ]);
        assert_eq!(trigram_opcode.get(1).unwrap(), &vec![
            "mov".to_string(),
            "pop".to_string(),
            "mov".to_string(),
        ]);

        let bigram_opcode: Vec<Vec<String>> = Code::n_gram_opcode(&elf, 2);
        assert_eq!(bigram_opcode.get(0).unwrap(), &vec![
            "xor".to_string(),
            "mov".to_string(),
        ]);
        assert_eq!(bigram_opcode.get(1).unwrap(), &vec![
            "mov".to_string(),
            "pop".to_string(),
        ]);
    }
}
