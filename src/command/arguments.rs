use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
pub struct ProjectArgs {
    #[arg(short = 'p', long = "project", help = "Path to project.json")]
    pub path: Option<PathBuf>,

    #[arg(long = "dry-run", help = "output command only")]
    pub dry_run: bool,
}

#[derive(Parser)]
pub struct ProjectAndBundleArgs {
    #[arg(short = 'p', long = "project", help = "Path to project.json")]
    pub path: Option<PathBuf>,

    #[arg(long = "dry-run", help = "output command only")]
    pub dry_run: bool,

    #[arg(
        short = 'b',
        long = "bundle",
        help = "Path to bundle json file",
        required = true
    )]
    pub bundle_path: Vec<PathBuf>,
}

impl ProjectArgs {
    pub fn project_path(&self) -> PathBuf {
        let target_path = resolve_path(
            self.path
                .as_ref()
                .unwrap_or(&PathBuf::from("./project.json")),
        );
        match target_path.is_file() {
            true => target_path,
            false => target_path.join("project.json"),
        }
    }
}

impl ProjectAndBundleArgs {
    pub fn project_path(&self) -> PathBuf {
        let target_path = resolve_path(
            self.path
                .as_ref()
                .unwrap_or(&PathBuf::from("./project.json")),
        );
        match target_path.is_file() {
            true => target_path,
            false => target_path.join("project.json"),
        }
    }

    pub fn bundle_path_items(&self) -> Vec<PathBuf> {
        self.bundle_path.iter().map(|x| resolve_path(x)).collect()
    }
}

fn resolve_path(target: &PathBuf) -> PathBuf {
    match fs::canonicalize(target) {
        Ok(path) => path,
        Err(_) => {
            eprintln!("Failed to resolve the file path: {}", target.display());
            eprintln!("Please confirm that the file exists.");
            eprintln!("You can also specify the path with the --project option.");
            std::process::exit(1);
        }
    }
}
