use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaRError {

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Portfolio is empty")]
    EmptyPortfolio,

    #[error("Invalid confidence level: {0}")]
    InvalidConfidenceLevel(f64),

    #[error("Insufficient data points")]
    InsufficientData,

    #[error("Invalid holding period: {0}")]
    InvalidHoldingPeriod(u32),
}

pub type Result<T> = std::result::Result<T, VaRError>;



