//! Defines the general set of error types in Coaster.

use std::{error, fmt};
use std::fmt::Display;

#[derive(Debug)]
/// Defines the set of available Coaster error types.
pub enum Error {
    /// Failure related to the Framework implementation.
    Framework(crate::framework::Error),
    /// Failure related to the Tensor.
    Tensor(crate::tensor::Error),
    /// Failure at Plugin Operation.
    Plugin(crate::plugin::Error),
    /// Failure related to a Device.
    Device(crate::device::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Framework(ref err) => write!(f, "Framwork error: {}", err),
            Error::Tensor(ref err) => write!(f, "Tensor error: {}", err),
            Error::Plugin(ref err) => write!(f, "Plugin error: {}", err),
            Error::Device(ref err) => write!(f, "Device error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Framework(ref err) => Some(err),
            Error::Tensor(ref err) => Some(err),
            Error::Plugin(ref err) => Some(err),
            Error::Device(ref err) => Some(err),
        }
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(time_err: std::time::SystemTimeError) -> Self {
       Error::Device(crate::device::Error::SystemError(format!("{:?}", time_err))) 
    }
}
