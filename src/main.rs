use clap::{Parser, Args, ArgGroup, Subcommand};
use std::process;

use obfuseval::Property;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Setup(SetupArgs),
    Evaluate(EvaluateArgs),
}

#[derive(Debug, Args)]
#[clap(group(ArgGroup::new("mode").required(false)))]
struct SetupArgs {
    #[clap(long, group = "mode")]
    compile_code: bool,
    #[clap(long, group = "mode")]
    obfuscate_code: bool,
    property_name: String,
}

#[derive(Debug, Args)]
#[clap(group(ArgGroup::new("mode").required(false)))]
struct EvaluateArgs {
    #[clap(long, group = "mode")]
    test_pass_rate: bool,
    #[clap(long, group = "mode")]
    code_distance_mean: bool,
    property_name: String,
}

fn main() -> anyhow::Result<()> {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Setup(args) => {
            let config: Property = Property::from_propertry_name(&args.property_name)?;
            let step: (bool, bool) = (args.compile_code, args.obfuscate_code);

            if let Err(e) = obfuseval::setup(config, step) {
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        },
        Commands::Evaluate(args) => {
            let config: Property = Property::from_propertry_name(&args.property_name)?;
            let step = (args.test_pass_rate, args.code_distance_mean);
            
            if let Err(e) = obfuseval::evaluate(config, step) {
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        },
    }

    Ok(())
}
