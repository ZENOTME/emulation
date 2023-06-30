use thiserror::Error;

pub type ExchangeResult<T> = std::result::Result<T, ExchangeError>;

#[derive(Debug, Error)]
pub enum ExchangeError {
    // This is a placeholder for now.
}
