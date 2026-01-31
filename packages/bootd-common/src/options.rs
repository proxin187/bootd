use crate::error::UefiError;

use alloc::string::{String, ToString};

use uefi::proto::loaded_image::LoadedImage;
use uefi::boot;


pub struct Options {
    options: String,
}

impl Options {
    pub fn new() -> Result<Option<Options>, UefiError> {
        let protocol = boot::open_protocol_exclusive::<LoadedImage>(boot::image_handle())?;

        Ok(protocol.load_options_as_cstr16().map(|options| Options { options: options.to_string() }).ok())
    }

    pub fn flag(&self, flag: &str) -> bool {
        self.options.contains(flag)
    }

    pub fn arg<'a>(&'a self, arg: &str) -> Option<&'a str> {
        let index = self.options.find(arg)?;

        self.options[index + arg.len()..].split(' ').next()
    }
}


