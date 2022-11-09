use thiserror::Error;

pub mod code_distance_mean;
pub mod test_pass_rate;

#[derive(Debug, Error)]
pub enum EvaluateError {
    #[error("FailTestError: {0}")]
    FailTestError(anyhow::Error),
    #[error("RunCodeError: {0}")]
    RucCodeError(anyhow::Error),
}
