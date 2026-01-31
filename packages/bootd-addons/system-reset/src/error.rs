use bootd_common::error::UefiError;

use alloc::string::String;


#[derive(Debug)]
pub enum Error {
    Uefi(UefiError),
    InvalidResetType(String),
    NoResetType,
    NoOptions,
}

impl<T: core::fmt::Debug> From<uefi::Error<T>> for Error {
    fn from(err: uefi::Error<T>) -> Self {
        Error::Uefi(UefiError::from(err))
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::Uefi(err) => f.write_fmt(format_args!("uefi error: {:?}", err)),
            Error::InvalidResetType(reset) => f.write_fmt(format_args!("invalid reset type: {}", reset)),
            Error::NoResetType => f.write_str("no reset type found with LoadOptions"),
            Error::NoOptions => f.write_str("no options found with LoadOptions"),
        }
    }
}

impl core::error::Error for Error {}


