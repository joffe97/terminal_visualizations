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

    #[error("string cannot be made out of given object")]
    StringConversionError,

    #[error("index is out of range")]
    IndexError,

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ImageError(#[from] image::ImageError),

    #[error(transparent)]
    Mp4Error(#[from] mp4::Error),
}
