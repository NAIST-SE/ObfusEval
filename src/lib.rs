//! # ObfusEval
//!
//!  ObfusEval is a tool to evaluate the reliability of code obfuscating transformations.

mod command;
pub mod model;

use anyhow::Result;
use clap::Parser;
use command::{
    // adjust_code::AdjustCodeCommand,
    list_obfuscation::ListObfuscationCommand,
    obfuscate::ObfuscateCommand,
    Command,
    CommandLineInterface,
    Commands,
};

pub fn run() -> Result<()> {
    let cli: CommandLineInterface = CommandLineInterface::parse();

    match cli.command {
        Commands::Obfuscate(args) => ObfuscateCommand::from(args).run(),
        // Commands::AdjustCode(args) => AdjustCodeCommand::from(args).run(),
        Commands::ListObfuscation(args) => ListObfuscationCommand::from(args).run(),
    }
}
