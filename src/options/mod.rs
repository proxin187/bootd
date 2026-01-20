use crate::error::Error;

use alloc::string::{String, ToString};
use alloc::boxed::Box;

use uefi::proto::loaded_image::LoadedImage;
use uefi::fs::PathBuf;
use uefi::{CString16, boot};


pub struct Options {
    options: String,
}

impl Options {
    pub fn new() -> Result<Options, Error> {
        let protocol = boot::open_protocol_exclusive::<LoadedImage>(boot::image_handle())
            .map_err(|err| Error::OpenProtocol(Box::new(err)))?;

        Ok(Options {
            options: protocol.load_options_as_cstr16().map_err(|_| Error::LoadOptions)?.to_string(),
        })
    }

    pub fn profiles(&self) -> Result<PathBuf, Error> {
        let index = self.options.find("-bootd.profiles=").ok_or(Error::NoProfilesPath)?;

        let path = self.options[index + 16..].split(' ').next().ok_or(Error::NoProfilesPath)?;

        Ok(PathBuf::from(CString16::try_from(path).map_err(|_| Error::NoProfilesPath)?))
    }
}


