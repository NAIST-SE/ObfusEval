//! # ObfusEval
//!
//!  ObfusEval is a tool to evaluate the reliability of code obfuscating transformations.

mod command;
pub mod model;

use anyhow::Result;
use clap::Parser;
use command::{
    adjust_code::AdjustCodeCommand, obfuscate::ObfuscateCommand, Command, CommandLineInterface,
    Commands,
};

pub fn run() -> Result<()> {
    let cli: CommandLineInterface = CommandLineInterface::parse();

    let cmd: Box<dyn Command> = match cli.command {
        Commands::Obfuscate(args) => Box::new(ObfuscateCommand::new(args)),
        Commands::AdjustCode(args) => Box::new(AdjustCodeCommand::new(args)),
    };

    cmd.run()
}
