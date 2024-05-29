use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod adjust_code;
pub mod obfuscate;

pub trait Command {
    fn run(&self) -> Result<()>;
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CommandLineInterface {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Obfuscate(obfuscate::Args),
    #[clap(hide(true))]
    AdjustCode(adjust_code::Args),
}
