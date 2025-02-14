use super::{Command, ProjectAndBundleArgs, ProjectArgs};
use crate::{
    project::{Project, ProjectInfo},
    Load,
};
use anyhow::Result;
use std::path::PathBuf;

pub struct EvaluateCommand {
    project_path: PathBuf,
    bundle_path: Vec<PathBuf>,
    dry_run: bool,
}

pub struct EvaluateAllCommand {
    project_path: PathBuf,
    dry_run: bool,
}

impl From<&ProjectAndBundleArgs> for EvaluateCommand {
    fn from(args: &ProjectAndBundleArgs) -> Self {
        Self {
            project_path: args.project_path(),
            bundle_path: args.bundle_path_items(),
            dry_run: args.dry_run,
        }
    }
}

impl From<&ProjectArgs> for EvaluateAllCommand {
    fn from(args: &ProjectArgs) -> Self {
        Self {
            project_path: args.project_path(),
            dry_run: args.dry_run,
        }
    }
}

impl Command for EvaluateCommand {
    fn run(&self) -> Result<()> {
        let project: ProjectInfo = ProjectInfo::load(&self.project_path)?;

        for bundle in &self.bundle_path {
            project.evaluate(&bundle, &self.dry_run)?;
        }
        Ok(())
    }
}

impl Command for EvaluateAllCommand {
    fn run(&self) -> Result<()> {
        let project: ProjectInfo = ProjectInfo::load(&self.project_path)?;

        for bundle in &project.all_bundle_path()? {
            project.evaluate(&bundle, &self.dry_run)?;
        }
        Ok(())
    }
}
