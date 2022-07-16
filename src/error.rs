use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error occurred due to different dimensions")]
    DifferentDimensions,

    #[error("error occurred due to invalid dimensions")]
    InvalidDimensions,

    #[error("terminal was not found")]
    TerminalNotFound,

    #[error("the given value cannot be NAN")]
    NANError,

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ImageError(#[from] image::ImageError),
}
