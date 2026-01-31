use bootd_common::error::UefiError;
use uefi::fs::PathBuf;


#[derive(Debug)]
pub enum Error {
    Uefi(UefiError),
    Fmt(core::fmt::Error),
    InvalidProfilesDirectory(PathBuf),
    NoProfilesPath,
    DevicePathFailed,
    InvalidProfileFormat,
    NoProfiles,
    NoOptions,
    NoMode,
}

impl<T: core::fmt::Debug> From<uefi::Error<T>> for Error {
    fn from(err: uefi::Error<T>) -> Error {
        Error::Uefi(UefiError::from(err))
    }
}

impl From<core::fmt::Error> for Error {
    fn from(err: core::fmt::Error) -> Error {
        Error::Fmt(err)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::Uefi(err) => f.write_fmt(format_args!("uefi error: {:?}", err)),
            Error::Fmt(err) => f.write_fmt(format_args!("fmt error: {:?}", err)),
            Error::InvalidProfilesDirectory(path) => f.write_fmt(format_args!("invalid profiles directory: {}", path)),
            Error::NoProfilesPath => f.write_str("couldnt find 'bootd.profiles' with LoadOptions"),
            Error::DevicePathFailed => f.write_str("failed to find device path"),
            Error::InvalidProfileFormat => f.write_str("invalid profile format"),
            Error::NoProfiles => f.write_str("no profiles found"),
            Error::NoOptions => f.write_str("no options found with LoadOptions"),
            Error::NoMode => f.write_str("no console mode found"),
        }
    }
}

impl core::error::Error for Error {}


