use nom::Needed;

use crate::frame::components::FrameControl;
use crate::generators::SerializationError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// This library can't parse all subtypes yet.
    /// If you hit a frame subtype that isn't supported, this error will be thrown.
    /// The [FrameControl] header should be successfully parsed in all scenarios and can be used
    /// for debugging.
    /// The remaining data is passed as second parameter and can be used for debugging.
    #[error("This frame subtype isn't handled yet: {:?} ({:?})", .0.frame_subtype, .0.frame_type)]
    UnhandledFrameSubtype(FrameControl, Vec<u8>),
    #[error("A parsing failure occurred: \n{}\ndata: {:?}", .0, .1)]
    ParseFailure(String, Vec<u8>),
    #[error("A serializing faliure occurred: \n{}", .0)]
    SerializeFailure(String),
    #[error("There wasn't enough data. {}", .0)]
    Incomplete(String),

    #[error("Libwifi cannot handle this specific protocol yet: {}", .0)]
    UnhandledProtocol(String),
}

impl From<nom::Err<nom::error::Error<&[u8]>>> for Error {
    /// Manually specify the conversion from a [nom::error::Error] to our own error.
    /// We need this conversion, since we work with slices.
    /// If nom's error is propagated through the program, we get lifetime issues as we can't hold
    /// ownership of that slice and thereby require a 'static.
    fn from(error: nom::Err<nom::error::Error<&[u8]>>) -> Self {
        match error {
            nom::Err::Incomplete(needed) => match needed {
                Needed::Size(size) => {
                    Error::Incomplete(format!("At least {} bytes are missing", size))
                }
                Needed::Unknown => Error::Incomplete(String::new()),
            },
            nom::Err::Failure(error) => Error::ParseFailure(
                format!(
                    "An error occured while parsing the data: nom::ErrorKind is {:?}",
                    error.code
                ),
                error.input.to_vec(),
            ),
            nom::Err::Error(error) => Error::ParseFailure(
                format!(
                    "An error occured while parsing the data: nom::ErrorKind is {:?}",
                    error.code
                ),
                error.input.to_vec(),
            ),
        }
    }
}

impl From<cookie_factory::GenError> for Error {
    fn from(error: cookie_factory::GenError) -> Self {
        match error {
            cookie_factory::GenError::BufferTooSmall(max_idx) => Error::SerializeFailure(
                format!("Provided buffer is too small! Minimum size is {}!", max_idx + 1)
            ),
            cookie_factory::GenError::CustomError(error_code) => {
                if let Ok(error) = SerializationError::try_from(error_code) {
                    Error::SerializeFailure(error.explain().to_owned())
                } else {
                    panic!("Unknown GenError::CustomError code! Are you using SerializationError.into()?");
                }
            },
            cookie_factory::GenError::InvalidOffset => Error::SerializeFailure(
                "Generator asked for inavlid index!".to_owned()
            ),
            cookie_factory::GenError::IoError(error) => Error::SerializeFailure(
                format!("Generator IO Error:\n{:?}", error)
            ),
            cookie_factory::GenError::NotYetImplemented => Error::SerializeFailure(
                "Generator not yet implemented!".to_owned(),
            ),
            // This should have been caught earlier.
            cookie_factory::GenError::BufferTooBig(_) => unreachable!(),
        }
    }
}