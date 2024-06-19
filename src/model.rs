use anyhow::Result;

use code_parser::AnalyzedCodeData;
use indicatif::ProgressBar;
use obfuscator::Obfuscation;
use std::path::PathBuf;

pub mod code;
pub mod code_organizer;
pub mod code_parser;
pub mod dataset;
pub mod obfuscator;

pub trait DatasetHandler {
    fn obfuscate_each_obfuscator(&self) -> Result<()>;
    fn obfuscate_each_code(&self, obfuscator: &(impl Obfuscator + std::marker::Sync))
        -> Result<()>;
    fn organize_each_code(&self) -> Result<()>;
}

pub trait CodeOrganizer {
    fn organize(src_path: &PathBuf, dst_path: &PathBuf) -> Result<()>;
}

pub trait Obfuscator {
    fn get_name(&self) -> &String;
    fn get_transformation_names(&self) -> Vec<&String>;

    fn obfuscate_by_all_obfuscation(
        &self,
        src_path: &PathBuf,
        dst_dir_path: &PathBuf,
        function_name: &str,
        pb: &Option<ProgressBar>,
    ) -> Result<()>;

    fn obfuscate(
        &self,
        src_path: &PathBuf,
        dst_dir_path: &PathBuf,
        function_name: &str,
        obfuscation: &Obfuscation,
        pb: &Option<ProgressBar>,
    ) -> Result<()>;
}

pub trait Parser<'a> {
    fn parse(source_code: &'a String) -> Result<Vec<AnalyzedCodeData<'a>>>;
}
