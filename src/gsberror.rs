#![deny(warnings)]
extern crate hyper;

use std::error;
use std::fmt;
use std::io::Error as ioError;


#[derive(Debug)]
/// Represents potential errors that may occur when performing lookups
pub enum GSBError {
    /// Represents Network errors, including access violations to the GSBL PI
    Network(hyper::error::Error),
    /// For when greater than gsbrs::url_limit urls are queried
    TooManyUrls,
    /// Signifies an error occured when converting the Response message into
    /// Statuses. Contains the String that it failed on.
    MalformedMessage(String),
    /// Represents a StatusCode indicating an error occurred when querying the GSBL API
    HTTPStatusCode(hyper::status::StatusCode),
    /// Wraps a std::io::Error
    IOError(ioError),
}

impl fmt::Display for GSBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GSBError::Network(ref err) => write!(f, "Network error: {}", err),
            GSBError::TooManyUrls => write!(f, "GSB API requires < 500 urls"),
            GSBError::MalformedMessage(ref string) => {
                write!(f,
                       "There was an unexpected value in the GSB response, please file a bug!
                       \
                        String found before error: {}",
                       string)
            }
            GSBError::HTTPStatusCode(sc) => write!(f, "Expected 200 Status Code, found: {}", sc),
            GSBError::IOError(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl error::Error for GSBError {
    fn description(&self) -> &str {
        match *self {
            GSBError::Network(ref err) => err.description(),
            GSBError::TooManyUrls => "GSB API requires < 500 urls",
            GSBError::MalformedMessage(_) => {
                "There was an unexpected value in the GSB response, please file a bug!"
            }
            GSBError::HTTPStatusCode(_) => "Expected HTTP StatusCode 200",
            GSBError::IOError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            GSBError::Network(ref err) => Some(err),
            GSBError::TooManyUrls => None,
            GSBError::MalformedMessage(_) => None,
            GSBError::HTTPStatusCode(_) => None,
            GSBError::IOError(ref err) => Some(err),
        }
    }
}

impl From<hyper::error::Error> for GSBError {
    fn from(err: hyper::error::Error) -> GSBError {
        GSBError::Network(err)
    }
}

impl From<hyper::status::StatusCode> for GSBError {
    fn from(err: hyper::status::StatusCode) -> GSBError {
        GSBError::HTTPStatusCode(err)
    }
}

impl From<ioError> for GSBError {
    fn from(err: ioError) -> GSBError {
        GSBError::IOError(err)
    }
}
