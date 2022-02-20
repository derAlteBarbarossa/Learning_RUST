use image::error;
use std::convert::From;
use std::{fmt, io};

#[derive(Debug)]
pub enum ImagixError {
    FileIOError(String),
    UserInputError(String),
    ImageResizingError(String),
    SizeOptionError(String),
    ModeOptionError(String),
    //FormatError(String),
}

impl From<io::Error> for ImagixError {
    fn from(_error: io::Error) -> Self {
        ImagixError::FileIOError("Error in File I/O".to_string())
    }
}

// ErrorKind happens when sth like access policy goes wrong
impl From<io::ErrorKind> for ImagixError {
    fn from(_error: io::ErrorKind) -> Self {
        ImagixError::UserInputError("Error in User Input".to_string())
    }
}

impl From<error::ImageError> for ImagixError {
    fn from(_error: error::ImageError) -> Self {
        ImagixError::ImageResizingError("Error in Resizing Operation".to_string())
    }
}

impl fmt::Display for ImagixError {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(out, "{}", "Error Occured!".to_string())
    }
}
