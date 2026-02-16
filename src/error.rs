use std::num::ParseIntError;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("No Game")]
    NoGame,
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Invalid input")]
    InvalidInput,
    #[error("Parse int error")]
    ParseIntErr(#[from] ParseIntError),
    #[error("Already exploded")]
    AlreadyExploded,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Unknown command: {0}")]
    UnknownCmd(Arc<str>),
    #[error("Parse int error")]
    ParseIntErr(#[from] ParseIntError),
    #[error("Invalid custom size")]
    InvalidCustom,
}
