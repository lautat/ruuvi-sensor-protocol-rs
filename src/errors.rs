use core::fmt::{self, Display};

#[cfg(feature = "std")]
use std::error::Error;

/// Errors which can occur during parsing of the manufacturer specific data
#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    /// Manufacturer id does not match expected value
    UnknownManufacturerId(u16),
    /// Format of the data is not supported by this crate
    UnsupportedFormatVersion(u8),
    /// Length of the value does not match expected length of the format
    InvalidValueLength(u8, usize, usize),
    /// Format can not be determined from value due to it being empty
    EmptyValue,
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ParseError::UnknownManufacturerId(id) => write!(
                formatter,
                "Unknown manufacturer id {:#04X}, only 0x0499 is supported",
                id
            ),
            ParseError::UnsupportedFormatVersion(format_version) => write!(
                formatter,
                "Unsupported data format version {}, only version 3 is supported",
                format_version
            ),
            ParseError::InvalidValueLength(version, length, expected) => write!(
                formatter,
                "Invalid data length of {} for format version {}, expected {}",
                length, version, expected
            ),
            ParseError::EmptyValue => write!(formatter, "Empty value, expected at least one byte"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for ParseError {}
