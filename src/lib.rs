//! # ObfusEval
//! 
//!  ObfusEval is a tool to evaluate the reliability of code obfuscating transformations.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};

mod builder;
mod evaluate;
mod program;
mod record;
mod setup;

use builder::Obfuscator;
use program::Dataset;
use record::TransformationUnitRecord;


#[derive(Deserialize, Serialize, Debug)]
pub struct Property {
    name: String,
    dataset: PathBuf,
    obfuscator: PathBuf,
}

impl Property {
    /// Returns a given name Property instance from deserialized property.json.
    fn load() -> Result<Vec<Property>> {
        let path = Path::new("properties.json").canonicalize()?;
        let file: File = File::open(path)?;
        Ok(serde_json::from_reader(file).unwrap())
    }

    /// Returns the Property instance from deserialized property.json.
    pub fn from_propertry_name(name: &String) -> Result<Property> {
        let configures = Property::load().context("Failed Open property.json")?;
        for c in configures {
            if c.name != *name {
                continue;
            }

            return Ok(Property {
                name: c.name,
                dataset: PathBuf::from("dataset").join(c.dataset),
                obfuscator: PathBuf::from("obfuscator")
                    .join(c.obfuscator)
                    .with_extension("json"),
            });
        }

        return Err(anyhow!("Property not found"));
    }
}

/// Compiles or Obfuscates codes in dataset.
pub fn setup(config: Property, step: (bool, bool)) -> Result<()> {
    let dataset: Dataset = Dataset::new(config.dataset, config.name)?;
    let obfuscator: Obfuscator = Obfuscator::new(config.obfuscator)?;
    dataset.init(&obfuscator.transformations_name())?;

    match step {
        (true, _) => setup::compile_code(&obfuscator, &dataset)?,
        (_, true) => setup::obfuscate_code(&obfuscator, &dataset)?,
        (false, false) => {
            setup::compile_code(&obfuscator, &dataset)?;
            setup::obfuscate_code(&obfuscator, &dataset)?;
        }
    }

    Ok(())
}

/// Evaluates obfuscator through test or compare opcode to original.
pub fn evaluate(config: Property, step: (bool, bool)) -> Result<()> {
    let dataset: Dataset = Dataset::new(config.dataset, config.name)?;
    let obfuscator: Obfuscator = Obfuscator::new(config.obfuscator)?;
    dataset.init(&obfuscator.transformations_name())?;

    let step = if step == (false, false) {
        (true, true)
    } else {
        step
    };

    let records: Vec<TransformationUnitRecord> = obfuscator
        .transformations
        .iter()
        .map(|t| TransformationUnitRecord::new(t, &dataset, &step))
        .collect();

    dbg!(&records);
    let dst = dataset.result_dir().join("result").with_extension("json");
    TransformationUnitRecord::output(&records, dst);

    Ok(())
}
