use crate::project::Project;

use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;

use super::{BundleInfo, CompilerOption, ObfuscationBundleInfo, ObfuscationInfo};

#[derive(Debug, Deserialize)]
struct BundleInfoSerializeModel {
    name: String,
    compiler: String,
    optimization: Vec<String>,
    obfuscation: Option<Vec<ObfuscationBundleInfoSerializeModel>>,
}

#[derive(Debug, Deserialize)]
struct ObfuscationBundleInfoSerializeModel {
    name: String,
    transformation_set: Vec<ObfuscationInfoSerializeModel>,
}

#[derive(Debug, Deserialize)]
struct ObfuscationInfoSerializeModel {
    obfuscator: String,
    transformation_key: String,
}

impl<'in_project> BundleInfo<'in_project> {
    pub fn load<T>(path: &PathBuf, project: &'in_project T) -> Result<Self>
    where
        T: Project,
    {
        let json = std::fs::File::open(path)?;
        let value: BundleInfoSerializeModel = serde_json::from_reader(json)?;

        let compiler = match project.get_compiler(&value.compiler) {
            Some(x) => x,
            None => {
                eprintln!("Compiler not found: {}", &value.compiler);
                std::process::exit(1);
            }
        };

        let mut obfuscation_bundle: Vec<Option<ObfuscationBundleInfo<'in_project>>> = vec![None];
        if let Some(x) = value.obfuscation {
            x.iter()
                .filter_map(|x| ObfuscationBundleInfo::new(x, project).ok())
                .for_each(|x| obfuscation_bundle.push(Some(x)));
        }

        Ok(BundleInfo {
            root: project.state_home().join(&value.name),
            name: value.name.clone(),
            temp_dir: project.cache_home().join(&value.name),
            compiler_option: CompilerOption::generate(
                compiler,
                value.optimization.iter().map(|s| s.as_str()).collect(),
            ),
            obfuscation: obfuscation_bundle,
        })
    }
}

impl<'in_project> ObfuscationBundleInfo<'in_project> {
    fn new<T: Project>(
        data: &ObfuscationBundleInfoSerializeModel,
        project: &'in_project T,
    ) -> Result<ObfuscationBundleInfo<'in_project>> {
        let obfuscation_bundle: Vec<ObfuscationInfo> = data
            .transformation_set
            .iter()
            .filter_map(|x| ObfuscationInfo::new(x, project).ok())
            .collect();

        Ok(ObfuscationBundleInfo {
            name: data.name.clone(),
            transformation_set: obfuscation_bundle,
        })
    }
}

impl<'in_project> ObfuscationInfo<'in_project> {
    fn new<T: Project>(
        data: &ObfuscationInfoSerializeModel,
        project: &'in_project T,
    ) -> Result<ObfuscationInfo<'in_project>> {
        let obfuscator = match project.get_obfuscator(&data.obfuscator) {
            Some(x) => x,
            None => {
                eprintln!("Obfuscator not found: {}", &data.obfuscator);
                std::process::exit(1);
            }
        };

        let transformation = match obfuscator.get_transformation(&data.transformation_key) {
            Some(x) => x,
            None => {
                eprintln!("Transformation not found: {}", &data.obfuscator);
                std::process::exit(1);
            }
        };

        Ok(ObfuscationInfo {
            obfuscator: obfuscator,
            transformation: transformation,
        })
    }
}
