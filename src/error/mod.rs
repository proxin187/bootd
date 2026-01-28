use alloc::boxed::Box;

use uefi::fs::PathBuf;
use uefi::CString16;


#[derive(Debug)]
pub enum Error {
    OpenProtocol(Box<dyn core::error::Error>),
    InvalidProfilesDirectory(PathBuf),
    LoadImageFailed(CString16),
    InputFailed(Option<uefi::Error>),
    OutputFailed(Option<uefi::Error>),
    LoadOptionsFailed,
    DevicePathFailed,
    StartImageFailed,
    NoProfilesPath,
    GetFileSystem,
    RootVolume,
    ReadEntryFailed,
    InvalidProfileFormat,
    NoProfiles,
    NoMode,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::OpenProtocol(err) => f.write_fmt(format_args!("failed to open protocol exclusively: {}", err)),
            Error::InvalidProfilesDirectory(path) => f.write_fmt(format_args!("invalid profiles directory: {}", path)),
            Error::LoadImageFailed(kernel) => f.write_fmt(format_args!("failed to load image: {}", kernel)),
            Error::InputFailed(err) => f.write_fmt(format_args!("input failed: {:?}", err)),
            Error::OutputFailed(err) => f.write_fmt(format_args!("output failed: {:?}", err)),
            Error::LoadOptionsFailed => f.write_str("failed to get load options: load options must be present and valid utf-16"),
            Error::DevicePathFailed => f.write_str("failed to create device path"),
            Error::StartImageFailed => f.write_str("failed to start image"),
            Error::NoProfilesPath => f.write_str("couldnt find 'bootd.profiles' with UEFI LoadOptions"),
            Error::GetFileSystem => f.write_str("failed to get image file system"),
            Error::RootVolume => f.write_str("failed to open the root file system volume"),
            Error::ReadEntryFailed => f.write_str("failed to read next entry on file system"),
            Error::InvalidProfileFormat => f.write_str("invalid profile format"),
            Error::NoProfiles => f.write_str("no profiles found"),
            Error::NoMode => f.write_str("no console mode found"),
        }
    }
}

impl core::error::Error for Error {}


