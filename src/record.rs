use thiserror::Error;

pub mod transformation_unit;
pub mod code_distance;
pub mod code_test;

#[derive(Debug, Error)]
pub enum CodeTestResultMatchError {
    #[error("ExitCodeError: {0}")]
    ExitCodeError(anyhow::Error),
    #[error("StdOutError: {0}")]
    StdOutError(anyhow::Error),
    #[error("StdErrError: {0}")]
    StdErrError(anyhow::Error),
}
