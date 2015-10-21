extern crate hyper;

use std::error;
use std::fmt;
use hyper::Error;
// We derive `Debug` because all types should probably derive `Debug`.
// This gives us a reasonable human readable description of `GSBError` values.
#[derive(Debug)]
pub enum GSBError {
    Network(hyper::error::Error),
}

impl fmt::Display for GSBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GSBError::Network(ref err) => write!(f, "Network error: {}", err),
        }
    }
}

impl error::Error for GSBError {

    fn description(&self) -> &str {
        match *self {
            GSBError::Network(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            GSBError::Network(ref err) => Some(err),
        }
    }
}

impl From<hyper::error::Error> for GSBError {
    fn from(err: hyper::error::Error) -> GSBError {
        GSBError::Network(err)
    }
}
