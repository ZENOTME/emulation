use daggy::WouldCycle;
use thiserror::Error;

use crate::exchange::error::ExchangeError;

pub type Result<T> = std::result::Result<T, ProcessError>;

#[derive(Debug, Error)]
pub enum ProcessError {
    #[error("Exchange: {0}")]
    Exchange(ExchangeError),
    #[error("Create error: {0}")]
    Create(String),
}

impl From<ExchangeError> for ProcessError {
    fn from(err: ExchangeError) -> Self {
        Self::Exchange(err)
    }
}

impl From<WouldCycle<()>> for ProcessError {
    fn from(_: WouldCycle<()>) -> Self {
        Self::Create("The buidler graph should not cycle".to_string())
    }
}
