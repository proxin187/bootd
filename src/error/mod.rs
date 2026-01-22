use alloc::boxed::Box;

use uefi::fs::PathBuf;


#[derive(Debug)]
pub enum Error {
    OpenProtocol(Box<dyn core::error::Error>),
    InvalidProfilesDirectory(PathBuf),
    LoadOptionsFailed,
    NoProfilesPath,
    GetFileSystem,
    RootVolume,
    ReadEntryFailed,
    InvalidProfileFormat,
    NoProfiles,
    InputFailed,
    OutputFailed,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::OpenProtocol(err) => f.write_fmt(format_args!("failed to open protocol exclusively: {}", err)),
            Error::InvalidProfilesDirectory(path) => f.write_fmt(format_args!("invalid profiles directory: {}", path)),
            Error::LoadOptionsFailed => f.write_str("failed to get load options: load options must be present and valid utf-16"),
            Error::NoProfilesPath => f.write_str("couldnt find 'bootd.profiles' with UEFI LoadOptions"),
            Error::GetFileSystem => f.write_str("failed to get image file system"),
            Error::RootVolume => f.write_str("failed to open the root file system volume"),
            Error::ReadEntryFailed => f.write_str("failed to read next entry on file system"),
            Error::InvalidProfileFormat => f.write_str("invalid profile format"),
            Error::NoProfiles => f.write_str("no profiles found"),
            Error::InputFailed => f.write_str("input failed"),
            Error::OutputFailed => f.write_str("output failed"),
        }
    }
}

impl core::error::Error for Error {}


