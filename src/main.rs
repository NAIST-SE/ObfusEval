use anyhow::Result;
use clap::{Parser, Subcommand};
use command::{obfuscate::ObfuscateCommandCore, Command};

mod command;
mod model;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(
        long = "use-docker-compose",
        help = "Execute command via docker service",
        default_value_t = false
    )]
    use_docker_compose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Obfuscate(command::obfuscate::ObfuscateCommand),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Obfuscate(cmd_args) => {
            let cmd = ObfuscateCommandCore::new(cmd_args);
            cmd.run(cli.use_docker_compose)
        }
    }
}
