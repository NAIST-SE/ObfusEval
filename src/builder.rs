use thiserror::Error;

pub mod service;
pub mod obfuscator;

#[derive(Debug, Error)]
pub enum BuilderError {
    #[error("RunDockerError: {0}")]
    RunDockerServiceError(anyhow::Error),
}