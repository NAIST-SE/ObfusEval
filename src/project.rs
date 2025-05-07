use serde::Deserialize;

use anyhow::Context;
use duct::cmd;

use crate::bundle::source::Source;
use crate::bundle::{Bundle, BundleInfo};
use crate::code_processor::compiler::Compiler;
use crate::code_processor::obfuscator::Obfuscator;

mod initialize;
mod load;

pub trait Project {
    fn config_home(&self) -> &std::path::PathBuf;
    fn cache_home(&self) -> &std::path::PathBuf;
    fn data_home(&self) -> &std::path::PathBuf;
    fn state_home(&self) -> &std::path::PathBuf;

    fn source_root(&self) -> &std::path::PathBuf;
    fn codeset_definition(&self) -> &std::path::PathBuf;

    fn compiler(&self) -> &Option<Vec<Box<dyn Compiler>>>;
    fn obfuscator(&self) -> &Option<Vec<Box<dyn Obfuscator>>>;

    fn bundles(&self) -> Option<Vec<BundleInfo>>;

    fn wake_up(&self, dry_run: &bool) -> anyhow::Result<()>;

    fn load_bundle(&self, bundle_path: &std::path::PathBuf) -> BundleInfo
    where
        Self: Sized,
    {
        match BundleInfo::load(bundle_path, self) {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Failed to load bundle file: {}", &bundle_path.display());
                std::process::exit(1);
            }
        }
    }

    fn get_compiler(&self, name: &str) -> Option<&Box<dyn Compiler>> {
        match self.compiler() {
            Some(compiler) => compiler.iter().find(|&x| x.name() == name),
            None => None,
        }
    }

    fn get_obfuscator(&self, name: &str) -> Option<&Box<dyn Obfuscator>> {
        match self.obfuscator() {
            Some(obfuscator) => obfuscator.iter().find(|&x| x.label() == name),
            None => None,
        }
    }

    fn build(&self, bundle_path: &std::path::PathBuf, dry_run: &bool) -> anyhow::Result<()> {
        let (_, _) = (bundle_path, dry_run);
        Ok(())
    }
    fn evaluate(&self, bundle_path: &std::path::PathBuf, dry_run: &bool) -> anyhow::Result<()> {
        let (_, _) = (bundle_path, dry_run);
        Ok(())
    }
}

// trait Initialize {
//     fn initialize(&self, dry_run: &bool) -> anyhow::Result<()>;
//     fn is_initialized(&self) -> bool;
// }

trait Docker {
    fn docker_root_path(&self) -> &std::path::PathBuf;

    fn docker_compose_up(&self, dry_run: &bool) -> anyhow::Result<()> {
        if !self.is_docker_availabe() {
            return Ok(());
        }

        if self.is_docker_compose_up() {
            return Ok(());
        }

        if *dry_run {
            println!("docker compose up -d");
            return Ok(());
        }

        cmd("docker", ["compose", "up", "-d"])
            .dir(&self.docker_root_path())
            .env("PWD", &self.docker_root_path().display().to_string())
            .read()?;
        Ok(())
    }

    fn is_docker_compose_up(&self) -> bool {
        cmd("docker", ["compose", "ps", "--status", "running"])
            .pipe(cmd("wc", ["-l"]))
            .dir(&self.docker_root_path())
            .read()
            .unwrap()
            .starts_with("1")
            == false
    }

    fn is_docker_availabe(&self) -> bool {
        // dockerコマンドがない場合はここで落とす
        self.docker_root_path().join("docker-compose.yml").exists()
    }
}

#[derive(Debug)]
pub struct ProjectInfo {
    name: String,
    root: std::path::PathBuf,
    path: ProjectPathInfo,
    source: ProjectSourceInfo,
    compiler: Option<Vec<Box<dyn Compiler>>>,
    obfuscator: Option<Vec<Box<dyn Obfuscator>>>,
}

#[derive(Debug, Clone, Deserialize)]
struct ProjectPathInfo {
    config_home: std::path::PathBuf,
    cache_home: std::path::PathBuf,
    data_home: std::path::PathBuf,
    state_home: std::path::PathBuf,
}

#[derive(Debug, Deserialize)]
struct ProjectSourceInfo {
    repository: String,
    branch: String,
    data_codeset: std::path::PathBuf,
    data_src: std::path::PathBuf,
    data_toolbox: std::path::PathBuf,
}

impl Docker for ProjectInfo {
    fn docker_root_path(&self) -> &std::path::PathBuf {
        &self.root
    }
}

impl Project for ProjectInfo {
    fn config_home(&self) -> &std::path::PathBuf {
        &self.path.config_home
    }
    fn cache_home(&self) -> &std::path::PathBuf {
        &self.path.cache_home
    }
    fn data_home(&self) -> &std::path::PathBuf {
        &self.path.data_home
    }
    fn state_home(&self) -> &std::path::PathBuf {
        &self.path.state_home
    }

    fn source_root(&self) -> &std::path::PathBuf {
        &self.source.data_src
    }
    fn codeset_definition(&self) -> &std::path::PathBuf {
        &self.source.data_codeset
    }

    fn compiler(&self) -> &Option<Vec<Box<dyn Compiler>>> {
        &self.compiler
    }
    fn obfuscator(&self) -> &Option<Vec<Box<dyn Obfuscator>>> {
        &self.obfuscator
    }

    fn wake_up(&self, dry_run: &bool) -> anyhow::Result<()> {
        // self.initialize(dry_run)
        //     .with_context(|| "Failed to initialize")?;
        self.docker_compose_up(dry_run)
            .with_context(|| "Failed to execute docker compose up")?;
        Ok(())
    }

    fn bundles(&self) -> Option<Vec<BundleInfo>> {
        let binding = self.all_bundle_path();
        let paths = match &binding {
            Ok(paths) => paths,
            Err(_) => return None,
        };

        let result: Vec<BundleInfo> = paths.iter().map(|path| self.load_bundle(path)).collect();

        match !result.is_empty() {
            true => Some(result),
            false => None,
        }
    }

    fn build(&self, bundle_path: &std::path::PathBuf, dry_run: &bool) -> anyhow::Result<()> {
        self.wake_up(dry_run)?;

        let bundle = self.load_bundle(&bundle_path);
        let sources = bundle.load_source_info(&self.source_root(), self.codeset_definition())?;

        // sources.par_iter().for_each(|source| {
        //     source.build(&self.docker_root_path(), dry_run).unwrap();
        // });

        for source in sources {
            // dbg!(&source);
            // 必要なディレクトリを作成

            // ビルド
            source.build(&self.docker_root_path(), dry_run)?;
        }

        Ok(())
    }

    fn evaluate(&self, bundle_path: &std::path::PathBuf, dry_run: &bool) -> anyhow::Result<()> {
        let _ = bundle_path;
        let _ = dry_run;
        todo!()
    }
}
