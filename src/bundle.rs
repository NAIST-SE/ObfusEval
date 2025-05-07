use std::path::PathBuf;

use source::{Source, SourceInfo, SourceInfoSerializeModel};

use crate::code_processor::{
    compiler::Compiler,
    obfuscator::{Obfuscator, TransformationInfo},
};

pub mod load;
pub mod source;

pub trait Bundle<'in_project> {
    fn root(&self) -> &PathBuf;

    fn name(&self) -> &str;

    fn temp_dir(&self) -> &PathBuf;

    fn compiler_option(&self) -> &Vec<CompilerOption<'in_project>>;

    fn optimization_items(&self) -> Vec<&str>;

    fn obfuscation(&self) -> &Vec<Option<ObfuscationBundleInfo<'in_project>>>;

    fn has_obfuscation(&self) -> bool {
        self.obfuscation().iter().any(|x| x.is_some())
    }

    fn cache_home(&self, obfuscation_bundle_name: Option<&str>) -> PathBuf {
        match obfuscation_bundle_name {
            Some(bundle_name) => self.temp_dir().join(bundle_name),
            None => self.temp_dir().to_path_buf(),
        }
    }
    fn data_home(
        &self,
        optimization: &str,
        enable_test: &bool,
        obfuscation_bundle_name: Option<&str>,
    ) -> PathBuf {
        let name = match *enable_test {
            true => format!("test-{}", optimization),
            false => optimization.to_string(),
        };
        let base = self.root().join(name);
        match obfuscation_bundle_name {
            Some(bundle_name) => base.join(bundle_name),
            None => base,
        }
    }
    fn obfuscated_source_home(&self, obfuscation_bundle_name: Option<&str>) -> PathBuf {
        let base = self.root().join("source");
        match obfuscation_bundle_name {
            Some(bundle_name) => base.join(bundle_name),
            None => base,
        }
    }

    fn load_source_info(
        &self,
        code_root_path: &std::path::PathBuf,
        codeset_definition: &std::path::PathBuf,
    ) -> anyhow::Result<Vec<impl Source>>;
}

#[derive(Debug)]
pub struct BundleInfo<'in_project> {
    root: PathBuf,
    name: String,
    temp_dir: PathBuf,
    compiler_option: Vec<CompilerOption<'in_project>>,
    obfuscation: Vec<Option<ObfuscationBundleInfo<'in_project>>>,
}

#[derive(Debug, Clone)]
pub struct CompilerOption<'in_project> {
    compiler: &'in_project Box<dyn Compiler>,
    optimization: String,
    enable_coverage: bool,
    enable_test: bool,
}

#[derive(Debug, Clone)]
pub struct ObfuscationBundleInfo<'in_project> {
    name: String,
    transformation_set: Vec<ObfuscationInfo<'in_project>>,
}

#[derive(Clone)]
struct ObfuscationInfo<'in_project> {
    obfuscator: &'in_project Box<dyn Obfuscator>,
    transformation: &'in_project TransformationInfo,
}

impl<'in_project> Bundle<'in_project> for BundleInfo<'in_project> {
    fn root(&self) -> &PathBuf {
        &self.root
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn temp_dir(&self) -> &PathBuf {
        &self.temp_dir
    }

    fn compiler_option(&self) -> &Vec<CompilerOption<'in_project>> {
        &self.compiler_option
    }

    fn optimization_items(&self) -> Vec<&str> {
        self.compiler_option
            .iter()
            .map(|x| x.optimization())
            .collect()
    }

    fn obfuscation(&self) -> &Vec<Option<ObfuscationBundleInfo<'in_project>>> {
        &self.obfuscation
    }

    fn load_source_info(
        &self,
        code_root_path: &std::path::PathBuf,
        codeset_definition: &std::path::PathBuf,
    ) -> anyhow::Result<Vec<impl Source>> {
        let json = std::fs::File::open(codeset_definition)?;
        let value: Vec<SourceInfoSerializeModel> = serde_json::from_reader(json)?;

        Ok(value
            .iter()
            .map(|x| x.normalize(&code_root_path))
            .flat_map(|x| SourceInfo::generate(&self, &x))
            .collect())
    }
}

impl<'in_project> CompilerOption<'in_project> {
    pub fn generate(
        compiler: &'in_project Box<dyn Compiler>,
        optimization: Vec<&str>,
    ) -> Vec<Self> {
        optimization
            .iter()
            .flat_map(|optimization| {
                [
                    Self {
                        compiler: compiler,
                        optimization: optimization.to_string(),
                        enable_coverage: true,
                        enable_test: false,
                    },
                    Self {
                        compiler: compiler,
                        optimization: optimization.to_string(),
                        enable_coverage: false,
                        enable_test: true,
                    },
                ]
            })
            .collect()
    }

    pub fn compiler(&self) -> &Box<dyn Compiler> {
        &self.compiler
    }
    pub fn optimization(&self) -> &str {
        &self.optimization
    }
    pub fn optimization_flag(&self) -> String {
        format!("-{}", self.optimization)
    }
    pub fn enable_coverage(&self) -> &bool {
        &self.enable_coverage
    }
    pub fn enable_test(&self) -> &bool {
        &self.enable_test
    }
}

impl<'in_project> std::fmt::Debug for ObfuscationInfo<'in_project> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ Obfusactor: {}, Transformation: {} }}",
            self.obfuscator.label(),
            self.transformation.display_name
        )
    }
}

impl<'in_project> ObfuscationBundleInfo<'in_project> {
    fn name(&self) -> &str {
        &self.name
    }
}
