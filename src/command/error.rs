use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("empty input")]
    EmptyInput,

    #[error("{0}: command not found")]
    UnknownCommand(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}