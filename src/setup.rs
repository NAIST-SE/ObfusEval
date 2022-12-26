//! # Setup
//! 
//! This module is a framework compiling or obfuscate C lang code through docker service.

use anyhow::Result;
use console::style;
use std::process;

pub mod compile_code;
pub mod obfuscate_code;

use crate::builder::obfuscator::Obfuscator;
use crate::program::dataset::Dataset;

/// Compile code
pub fn compile_code(obfuscator: &Obfuscator, dataset: &Dataset) -> Result<()> {
    println!("{} Compiling code...", style("[+]").bold().dim());

    dataset.make_bin(&obfuscator.compiler())?;

    Ok(())
}

/// Obfuscate code
pub fn obfuscate_code(obfuscator: &Obfuscator, dataset: &Dataset) -> Result<()> {
    println!("{} Obfuscating code...", style("[+]").bold().dim());

    obfuscator
        .transformations
        .iter()
        .inspect(|t| {
            println!("{} {}", style(" * ").bold().dim(), t.name);
            if let Err(e) = dataset.make_obf_code(obfuscator, t) {
                eprintln!("{}", e);
                process::exit(1);
            }
        })
        .for_each(drop);

    Ok(())
}
