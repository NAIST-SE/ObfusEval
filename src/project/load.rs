use glob::glob;
use serde::Deserialize;

use crate::{
    code_processor::{
        compiler::Compiler, obfuscator::Obfuscator, ExecutorInfo, LoadCompiler, LoadObfuscator,
    },
    Load,
};

use super::{Project, ProjectInfo, ProjectPathInfo, ProjectSourceInfo};

#[derive(Debug, Deserialize)]
struct ProjectInfoSerializeModel {
    name: String,
    path: ProjectPathInfo,
    source: ProjectSourceInfo,
}

impl Load for ProjectInfo {
    fn load(path: &std::path::PathBuf) -> anyhow::Result<Self> {
        let json = std::fs::File::open(path)?;
        let value: ProjectInfoSerializeModel = serde_json::from_reader(json)?;

        // パスの正規化
        let root = std::fs::canonicalize(path.parent().unwrap())?;
        let path_info = ProjectPathInfo {
            config_home: root.join(&value.path.config_home),
            cache_home: root.join(&value.path.cache_home),
            data_home: root.join(&value.path.data_home),
            state_home: root.join(&value.path.state_home),
        };
        let source = ProjectSourceInfo {
            repository: value.source.repository.clone(),
            branch: value.source.branch.clone(),
            data_codeset: path_info.data_home.join(&value.source.data_codeset),
            data_src: path_info.data_home.join(&value.source.data_src),
            data_toolbox: path_info.data_home.join(&value.source.data_toolbox),
        };

        let compiler = load_compiler(&path_info.config_home);
        let obfuscator = load_obfuscator(&path_info.config_home);

        Ok(Self {
            name: value.name,
            root: root,
            path: path_info,
            source: source,
            compiler: compiler,
            obfuscator: obfuscator,
        })
    }
}

impl ProjectInfo {
    pub fn all_bundle_path(&self) -> anyhow::Result<Vec<std::path::PathBuf>> {
        json_paths(&format!(
            "{}*/*.bundle.json",
            self.config_home().join("bundle").display()
        ))
    }
}

pub fn json_paths(pattern: &str) -> anyhow::Result<Vec<std::path::PathBuf>> {
    let paths: Vec<std::path::PathBuf> = glob(&pattern)?.filter_map(Result::ok).collect();
    Ok(paths)
}

fn load_compiler(path: &std::path::PathBuf) -> Option<Vec<Box<dyn Compiler>>> {
    let paths = match json_paths(&format!("{}*.compiler.json", path.display())) {
        Ok(paths) => paths,
        Err(_) => return None,
    };

    let result: Vec<Box<dyn Compiler>> = paths
        .iter()
        .filter_map(|path| {
            let executor = match ExecutorInfo::load(&path) {
                Ok(executor) => executor,
                Err(_) => return None,
            };
            executor.to_compiler(&path).ok()
        })
        .collect();

    match !result.is_empty() {
        true => Some(result),
        false => None,
    }
}

fn load_obfuscator(path: &std::path::PathBuf) -> Option<Vec<Box<dyn Obfuscator>>> {
    let paths = match json_paths(&format!("{}*.obfuscator.json", path.display())) {
        Ok(paths) => paths,
        Err(_) => return None,
    };

    let result: Vec<Box<dyn Obfuscator>> = paths
        .iter()
        .filter_map(|path| {
            let executor = match ExecutorInfo::load(&path) {
                Ok(executor) => executor,
                Err(_) => return None,
            };
            executor.to_obfuscator(&path).ok()
        })
        .collect();

    match !result.is_empty() {
        true => Some(result),
        false => None,
    }
}
