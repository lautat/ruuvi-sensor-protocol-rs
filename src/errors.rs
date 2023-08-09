use core::fmt::{self, Display};

#[cfg(feature = "std")]
use std::error::Error;

/// Errors which can occur during parsing of the manufacturer specific data
#[derive(Clone, Debug, Eq, PartialEq)]
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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ParseError::UnknownManufacturerId(id) => write!(
                formatter,
                "Unknown manufacturer id {id:#04X}, only 0x0499 is supported"
            ),
            ParseError::UnsupportedFormatVersion(format_version) => write!(
                formatter,
                "Unsupported data format version {format_version}, only version 3 is supported"
            ),
            ParseError::InvalidValueLength(version, length, expected) => write!(
                formatter,
                "Invalid data length of {length} for format version {version}, expected {expected}"
            ),
            ParseError::EmptyValue => write!(formatter, "Empty value, expected at least one byte"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_error_has_default_traits() {
        crate::testing::type_has_default_traits::<ParseError>();
    }
}
