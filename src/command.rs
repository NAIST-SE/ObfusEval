use anyhow::Result;

pub mod obfuscate;

pub trait Command {
    fn run(&self, use_docker: bool) -> Result<()>;
}
