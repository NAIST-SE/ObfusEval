use duct::cmd;
use rs_algo::compare::LCSubstring;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

use crate::program::Code;

#[derive(Deserialize, Serialize, Debug)]
pub struct SimilarityRecord {
    simpson: f32,
    longest_common_substring: f32,
    // Write here that your metrics.
}

impl SimilarityRecord {
    pub fn new(elf: &PathBuf, alter_elf: &PathBuf) -> SimilarityRecord {
        let simpson = CodeSimilarity::simpson(&elf, &alter_elf);
        let lcsubstr_percentage = CodeSimilarity::longest_common_substring(&elf, &alter_elf);

        SimilarityRecord {
            simpson: simpson,
            longest_common_substring: lcsubstr_percentage,
        }
    }

    pub fn calc_distance_mean(records: Vec<(&PathBuf, &SimilarityRecord)>) -> Vec<(String, f32)> {
        let records_len: f32 = records.len() as f32;
        vec![
            ("simpson".to_string(), records.iter().map(|(_, r)| 1.0 - r.simpson).sum::<f32>() / records_len),
            ("lcsubstr".to_string(), records.iter().map(|(_, r)| 1.0 - r.longest_common_substring).sum::<f32>() / records_len),
        ]
    }
}

struct CodeSimilarity {}

impl CodeSimilarity {
    fn simpson(elf: &PathBuf, alter_elf: &PathBuf) -> f32 {
        let n = 3;

        let a: HashSet<Vec<String>> = HashSet::from_iter(Code::n_gram_opcode(elf, n));
        let b: HashSet<Vec<String>> = HashSet::from_iter(Code::n_gram_opcode(alter_elf, n));

        let numerator: f32 = a.intersection(&b).collect::<Vec<_>>().len() as f32;
        let denominator: f32 = if a.len() < b.len() { a.len() } else { b.len() } as f32;

        numerator / denominator
    }

    fn longest_common_substring(elf: &PathBuf, alter_elf: &PathBuf) -> f32 {
        let a = Code::opcode(elf).join(" ");
        let b = Code::opcode(alter_elf).join(" ");
        let denominator: f32 = if &a.len() <= &b.len() {
            b.len()
        } else {
            a.len()
        } as f32;
        let lcs = LCSubstring::new_substring(a, b);

        lcs.substring_len as f32 / denominator
    }

    // Write here that how to calculation your metrics.
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
