//! Errors thrown by the library

use alloc::string::String;
use core::error::Error;
use core::fmt;

/// Enum of errors returned by this library
#[derive(Debug)]
pub enum TrieError {
    NotFound(String),
}

impl Error for TrieError {}

impl fmt::Display for TrieError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TrieError::NotFound(ref msg) => write!(f, "{}", msg),
        }
    }
}
