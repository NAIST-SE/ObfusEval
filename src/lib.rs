//! # ObfusEval
//!
//!  ObfusEval is a tool to evaluate the reliability of code obfuscating transformations.

mod bundle;
mod code_processor;
mod command;
mod project;

use anyhow::Result;
use command::CommandLineArgsParser;

trait Load: Sized {
    fn load(path: &std::path::PathBuf) -> anyhow::Result<Self>;
}

pub fn run() -> Result<()> {
    CommandLineArgsParser::parse().execute()
}
