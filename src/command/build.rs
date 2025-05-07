use super::{Command, ProjectAndBundleArgs, ProjectArgs};

use crate::{
    project::{Project, ProjectInfo},
    Load,
};

use anyhow::Result;
use std::path::PathBuf;

pub struct BuildCommand {
    project_path: PathBuf,
    bundle_path: Vec<PathBuf>,
    dry_run: bool,
}

pub struct BuildAllCommand {
    project_path: PathBuf,
    dry_run: bool,
}

impl From<&ProjectAndBundleArgs> for BuildCommand {
    fn from(args: &ProjectAndBundleArgs) -> Self {
        Self {
            project_path: args.project_path(),
            bundle_path: args.bundle_path_items(),
            dry_run: args.dry_run,
        }
    }
}

impl From<&ProjectArgs> for BuildAllCommand {
    fn from(args: &ProjectArgs) -> Self {
        Self {
            project_path: args.project_path(),
            dry_run: args.dry_run,
        }
    }
}

impl Command for BuildCommand {
    fn run(&self) -> Result<()> {
        let project: ProjectInfo = ProjectInfo::load(&self.project_path)?;

        for bundle in &self.bundle_path {
            project.build(&bundle, &self.dry_run)?;
        }
        Ok(())
    }
}

impl Command for BuildAllCommand {
    fn run(&self) -> Result<()> {
        let project: ProjectInfo = ProjectInfo::load(&self.project_path)?;

        for bundle in &project.all_bundle_path()? {
            println!("Building bundle: {}", bundle.to_string_lossy());
            project.build(&bundle, &self.dry_run)?;
        }
        Ok(())
    }
}
