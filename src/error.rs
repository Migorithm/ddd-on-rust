use std::{error, fmt::Display};

pub type AnyError = dyn error::Error + Send + Sync + 'static;

#[derive(Debug)]
pub enum AggregateError {
    AggregateConflict,
    DatabaseConnectionError(Box<AnyError>),
    DeserializationError(Box<AnyError>),
    UnexpectedError(Box<AnyError>),
    CommandNotConvertible,
    NotFound,
}

impl error::Error for AggregateError {}

impl Display for AggregateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregateError::AggregateConflict => write!(f, "AggregateConflict"),
            AggregateError::DatabaseConnectionError(res) => write!(f, "{}", res),
            AggregateError::DeserializationError(res) => write!(f, "{}", res),
            AggregateError::UnexpectedError(res) => write!(f, "{}", res),
            AggregateError::CommandNotConvertible => write!(f, "CommandNotConvertible"),
            AggregateError::NotFound => write!(f, "NotFound"),
        }
    }
}
