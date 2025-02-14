use serde::Deserialize;

use super::{Bundle, BundleInfo, CompilerOption, ObfuscationBundleInfo};

pub trait Source<'in_project> {
    // Property
    fn name(&self) -> &String;
    fn source(&self) -> &Vec<std::path::PathBuf>;
    fn function_name(&self) -> &str;
    fn compiler_option(&self) -> &CompilerOption;
    fn obfuscator_option(&self) -> &Option<ObfuscationBundleInfo<'in_project>>;

    fn cache_home(&self) -> &std::path::PathBuf;
    fn data_home(&self) -> &std::path::PathBuf;
    fn obfuscated_source_home(&self) -> &std::path::PathBuf;

    fn binary_path(&self) -> std::path::PathBuf {
        self.data_home().join(self.name()).with_extension("elf")
    }
    fn obfuscated_source_path(&self) -> std::path::PathBuf {
        self.obfuscated_source_home()
            .join(self.name())
            .with_extension("c")
    }
    fn temp_obfuscated_source_path(&self) -> std::path::PathBuf {
        self.cache_home().join(self.name()).with_extension("c")
    }
    fn temp_obfuscated_binary_path(&self) -> std::path::PathBuf {
        self.cache_home().join(self.name()).with_extension("elf")
    }

    fn is_built(&self) -> bool {
        self.binary_path().exists()
    }

    // Method
    fn build(&self, working_directory: &std::path::PathBuf, dry_run: &bool) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct SourceInfo<'in_project> {
    name: String,
    source: Vec<std::path::PathBuf>,
    function_name: String,
    compiler_option: &'in_project CompilerOption<'in_project>,
    obfuscator_option: &'in_project Option<ObfuscationBundleInfo<'in_project>>,
    cache_home: std::path::PathBuf,
    data_home: std::path::PathBuf,
    obfuscated_source_home: std::path::PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct SourceInfoSerializeModel {
    directory: std::path::PathBuf,
    src: std::path::PathBuf,
    src_for_check: std::path::PathBuf,
    src_for_test: Vec<std::path::PathBuf>,
    function: String,
}

impl<'in_project> Source<'in_project> for SourceInfo<'in_project> {
    fn name(&self) -> &String {
        &self.name
    }

    fn source(&self) -> &Vec<std::path::PathBuf> {
        &self.source
    }

    fn function_name(&self) -> &str {
        &self.function_name
    }

    fn compiler_option(&self) -> &CompilerOption {
        self.compiler_option
    }

    fn obfuscator_option(&self) -> &Option<ObfuscationBundleInfo<'in_project>> {
        self.obfuscator_option
    }

    fn cache_home(&self) -> &std::path::PathBuf {
        &self.cache_home
    }

    fn data_home(&self) -> &std::path::PathBuf {
        &self.data_home
    }

    fn obfuscated_source_home(&self) -> &std::path::PathBuf {
        &self.obfuscated_source_home
    }

    fn build(&self, working_directory: &std::path::PathBuf, dry_run: &bool) -> anyhow::Result<()> {
        if self.is_built() {
            return Ok(());
        }

        std::fs::create_dir_all(&self.cache_home())?;
        std::fs::create_dir_all(&self.data_home())?;

        if let Some(obf_bundle) = self.obfuscator_option() {
            std::fs::create_dir_all(&self.obfuscated_source_home())?;

            obf_bundle
                .transformation_set
                .iter()
                .for_each(|obfuscation| {
                    let _ = obfuscation.obfuscator.obfuscate(
                        working_directory,
                        self.source(),
                        self.function_name(),
                        obfuscation.transformation,
                        &self.obfuscated_source_path(),
                        &self.temp_obfuscated_source_path(),
                        &self.temp_obfuscated_binary_path(),
                        dry_run,
                    );
                });
        }

        self.compiler_option().compiler().compile(
            &working_directory,
            self.source(),
            &self.binary_path(),
            self.compiler_option(),
            dry_run,
        )
    }
}

impl<'in_project> SourceInfo<'in_project> {
    pub fn generate(
        bundle: &'in_project BundleInfo,
        value: &SourceInfoSerializeModel,
    ) -> Vec<SourceInfo<'in_project>> {
        bundle
            .compiler_option()
            .iter()
            .flat_map(|compiler_option| {
                let source = match compiler_option.enable_test() {
                    true => value.source_for_test(),
                    false => value.source_for_check(),
                };

                bundle
                    .obfuscation()
                    .iter()
                    .map(|obfuscation_bundle| {
                        let obfuscation_bundle_name = match obfuscation_bundle {
                            Some(i) => Some(i.name()),
                            None => None,
                        };
                        let cache_home = bundle.cache_home(obfuscation_bundle_name);
                        let data_home = bundle.data_home(
                            &compiler_option.optimization(),
                            compiler_option.enable_test(),
                            obfuscation_bundle_name,
                        );
                        let obfuscated_source_home =
                            bundle.obfuscated_source_home(obfuscation_bundle_name);

                        Self {
                            name: value.name(),
                            source: source.clone(),
                            function_name: value.function.clone(),
                            compiler_option: compiler_option,
                            obfuscator_option: obfuscation_bundle,
                            cache_home: cache_home,
                            data_home: data_home,
                            obfuscated_source_home: obfuscated_source_home,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl SourceInfoSerializeModel {
    pub fn normalize(&self, root: &std::path::PathBuf) -> Self {
        let directory = resolve_path(root, &self.directory);
        let src = resolve_path(&directory, &self.src);
        let src_for_check = resolve_path(&directory, &self.src_for_check);
        let src_for_test = self
            .src_for_test
            .iter()
            .map(|x| resolve_path(&directory, &x))
            .collect();

        Self {
            directory: directory,
            src: src,
            src_for_check: src_for_check,
            src_for_test: src_for_test,
            function: self.function.clone(),
        }
    }

    fn name(&self) -> String {
        self.directory
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string()
    }

    fn source_for_check(&self) -> Vec<std::path::PathBuf> {
        vec![self.src.clone(), self.src_for_check.clone()]
    }

    fn source_for_test(&self) -> Vec<std::path::PathBuf> {
        [
            vec![self.src.clone()],
            self.src_for_test.iter().map(|x| x.clone()).collect(),
        ]
        .concat()
    }
}

fn resolve_path(root: &std::path::PathBuf, target: &std::path::PathBuf) -> std::path::PathBuf {
    match std::fs::canonicalize(root.join(&target)) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Failed to resolve path: {}", &target.display());
            std::process::exit(1);
        }
    }
}
