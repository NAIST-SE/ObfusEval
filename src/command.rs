mod arguments;
mod build;
mod evaluate;

use anyhow::Result;
use arguments::{ProjectAndBundleArgs, ProjectArgs};
use build::{BuildAllCommand, BuildCommand};
use clap::{Parser, Subcommand};

trait Command {
    fn run(&self) -> Result<()>;
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CommandLineArgsParser {
    #[command(subcommand)]
    command: Commands,
}

impl CommandLineArgsParser {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub fn execute(&self) -> Result<()> {
        match &self.command {
            Commands::Build(args) => BuildCommand::from(args).run(),
            Commands::BuildAll(args) => BuildAllCommand::from(args).run(),
            // Commands::Evaluate(args) => EvaluateCommand::from(args).run(),
            // Commands::EvaluateAll(args) => EvaluateAllCommand::from(args).run(),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    Build(ProjectAndBundleArgs),
    BuildAll(ProjectArgs),
    // Evaluate(ProjectAndBundleArgs),
    // EvaluateAll(ProjectArgs),
}
