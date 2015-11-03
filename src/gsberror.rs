#![deny(warnings)]
extern crate hyper;

use std::error;
use std::fmt;
use hyper::Error;
use hyper::status::StatusCode;
use std::io::Error as ioError;

#[derive(Debug)]
pub enum GSBError {
    Network(hyper::error::Error),
    TooManyUrls,
    MalformedMessage,
    HTTPStatusCode(hyper::status::StatusCode),
    IOError(ioError)
}



impl fmt::Display for GSBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GSBError::Network(ref err) => write!(f, "Network error: {}", err),
            GSBError::TooManyUrls => write!(f, "GSB API requires < 500 urls"),
            GSBError::MalformedMessage =>
                write!(f,
                       "There was an unexpected value in the GSB response, please file a bug!"),
            GSBError::HTTPStatusCode(sc)   =>
                write!(f, "Expected 200 Status Code, found: {}", sc),
            GSBError::IOError(ref err) => write!(f, "IO error: {}", err),
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
            GSBError::HTTPStatusCode(_) => "Expected HTTP StatusCode 200",
            GSBError::IOError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            GSBError::Network(ref err) => Some(err),
            GSBError::TooManyUrls => None,
            GSBError::MalformedMessage => None,
            GSBError::HTTPStatusCode(_)  => None,
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
