use serde::Deserialize;
use tigress_v3::TigressV3;

use super::{Executor, ExecutorInfo, LoadObfuscator};

pub mod tigress_v3;
// mod tigress_v4;
// mod ollvm;

pub trait Obfuscator: Executor {
    fn new(executor: ExecutorInfo, value: ObfuscatorSerializeModel) -> Self
    where
        Self: Sized;

    fn transformation_set(&self) -> &Vec<TransformationInfo>;

    fn args(
        &self,
        source: &[std::path::PathBuf],
        function_name: &str,
        transformation: &TransformationInfo,
        obfuscated_source_path: &std::path::PathBuf,
        temp_obfuscated_binary_path: &std::path::PathBuf,
    ) -> Vec<String>;

    fn get_transformation(&self, key: &str) -> Option<&TransformationInfo> {
        self.transformation_set()
            .iter()
            .find(|transformation| transformation.display_name == key)
    }

    fn has_post_process(&self) -> bool;

    fn pre_obfuscate(
        &self,
        working_directory: &std::path::PathBuf,
        source: &[std::path::PathBuf],
        function_name: &str,
        transformation: &TransformationInfo,
        temp_obfuscated_source_path: &std::path::PathBuf,
        dry_run: &bool,
    ) -> anyhow::Result<Vec<std::path::PathBuf>> {
        let (_, _, _, _, _) = (
            working_directory,
            function_name,
            transformation,
            temp_obfuscated_source_path,
            dry_run,
        );
        return Ok(source.to_vec());
    }

    fn obfuscate(
        &self,
        working_directory: &std::path::PathBuf,
        source: &[std::path::PathBuf],
        function_name: &str,
        transformation: &TransformationInfo,
        obfuscated_source_path: &std::path::PathBuf,
        temp_obfuscated_source_path: &std::path::PathBuf,
        temp_obfuscated_binary_path: &std::path::PathBuf,
        dry_run: &bool,
    ) -> anyhow::Result<()> {
        let source = self.pre_obfuscate(
            working_directory,
            source,
            function_name,
            transformation,
            temp_obfuscated_source_path,
            dry_run,
        )?;

        let args: Vec<String> = self.args(
            &source,
            function_name,
            transformation,
            match self.has_post_process() {
                true => temp_obfuscated_source_path,
                false => obfuscated_source_path,
            },
            temp_obfuscated_binary_path,
        );
        self.execute(&args, working_directory, dry_run)?;

        if temp_obfuscated_source_path.exists() {
            self.post_obfuscate(
                working_directory,
                function_name,
                temp_obfuscated_source_path,
                temp_obfuscated_binary_path,
                obfuscated_source_path,
                dry_run,
            )?;
        }

        Ok(())
    }

    fn post_obfuscate(
        &self,
        working_directory: &std::path::PathBuf,
        function_name: &str,
        temp_obfuscated_source_path: &std::path::PathBuf,
        temp_obfuscated_binary_path: &std::path::PathBuf,
        obfuscated_source_path: &std::path::PathBuf,
        dry_run: &bool,
    ) -> anyhow::Result<()> {
        let (_, _, _, _, _, _) = (
            working_directory,
            function_name,
            temp_obfuscated_source_path,
            temp_obfuscated_binary_path,
            obfuscated_source_path,
            dry_run,
        );
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct ObfuscatorSerializeModel {
    common_parameter: Vec<String>,
    transformation_set: Vec<TransformationInfo>,
}

#[derive(Debug, Deserialize)]
pub struct TransformationInfo {
    name: String,
    pub display_name: String,
    parameter: Vec<String>,
}

impl std::fmt::Debug for dyn Obfuscator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl LoadObfuscator for ExecutorInfo {
    fn to_obfuscator(self, path: &std::path::PathBuf) -> anyhow::Result<Box<dyn Obfuscator>> {
        let json = std::fs::File::open(path)?;
        let value: ObfuscatorSerializeModel = serde_json::from_reader(json)?;

        if self.name == "tigress" && self.version.starts_with("v3") {
            return Ok(Box::new(TigressV3::new(self, value)));
        }
        // if self.name == "tigress" && self.version.starts_with("v4") {
        //     return Ok(Box::new(TigressV4::new(self, value)));
        // }
        // if self.name == "ollvm" {
        //     return Ok(Box::new(Ollvm::new(self, value)));
        // }
        Err(anyhow::anyhow!("Unsupported obfuscator: {}", self.name))
    }
}

impl TransformationInfo {
    fn generate_parameter(&self, function_name: &str) -> Vec<String> {
        self.parameter
            .join(" ")
            .replace("--Functions=*", &format!("--Functions={}", function_name))
            .split_whitespace()
            .map(|x| x.to_string())
            .collect()
    }
}
