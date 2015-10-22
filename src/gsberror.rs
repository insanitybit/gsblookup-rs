#![deny(warnings)]
extern crate hyper;

use std::error;
use std::fmt;
use hyper::Error;

#[derive(Debug)]
pub enum GSBError {
    Network(hyper::error::Error),
    TooManyUrls,
    MalformedMessage,
}

impl fmt::Display for GSBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GSBError::Network(ref err) => write!(f, "Network error: {}", err),
            GSBError::TooManyUrls => write!(f, "GSB API requires < 500 urls"),
            GSBError::MalformedMessage =>
                write!(f,
                       "There was an unexpected value in the GSB response, please file a bug!"),
        }
    }
}

impl error::Error for GSBError {

    fn description(&self) -> &str {
        match *self {
            GSBError::Network(ref err) => err.description(),
            GSBError::TooManyUrls => "GSB API requires < 500 urls",
            GSBError::MalformedMessage =>
                "There was an unexpected value in the GSB response, please file a bug!",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            GSBError::Network(ref err) => Some(err),
            GSBError::TooManyUrls => None,
            GSBError::MalformedMessage => None,
        }
    }
}

impl From<hyper::error::Error> for GSBError {
    fn from(err: hyper::error::Error) -> GSBError {
        GSBError::Network(err)
    }
}
