use alloc::boxed::Box;


#[derive(Debug)]
pub enum Error {
    OpenProtocol(Box<dyn core::error::Error>),
    LoadOptions,
    NoProfilesPath,
    GetFileSystem,
    RootVolume,
    ReadEntry,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::OpenProtocol(err) => f.write_fmt(format_args!("failed to open protocol exclusively: {}", err)),
            Error::LoadOptions => f.write_str("failed to get load options: load options must be present and valid utf-16"),
            Error::NoProfilesPath => f.write_str("couldnt find 'bootd.profiles' with UEFI LoadOptions"),
            Error::GetFileSystem => f.write_str("failed to get image file system"),
            Error::RootVolume => f.write_str("failed to open the root file system volume"),
            Error::ReadEntry => f.write_str("failed to read next entry on file system"),
        }
    }
}

impl core::error::Error for Error {}


