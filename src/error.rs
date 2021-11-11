// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

#[derive(Debug)]
/// Indicates that some error occurred within the Iced backend
pub enum Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for Error {}
