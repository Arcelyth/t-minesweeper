use thiserror::Error;
use std::num::ParseIntError;

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
