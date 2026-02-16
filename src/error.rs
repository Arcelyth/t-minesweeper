use std::num::ParseIntError;
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
    #[error("Unknown command")]
    UnknownCmd,
    #[error("Parse int error")]
    ParseIntErr(#[from] ParseIntError),
    #[error("Invalid custom size")]
    InvalidCustom,
}
