use crate::error::Error;

use alloc::vec::Vec;

use uefi::proto::media::file::File;
use uefi::fs::PathBuf;
use uefi::{boot, CString16};


pub struct Profile {
    name: CString16,
    kernel: PathBuf,
    cmdline: CString16,
}

impl Profile {
}

pub fn load_profiles() -> Result<Vec<Profile>, Error> {
    let mut fs = boot::get_image_file_system(boot::image_handle())
        .map_err(|_| Error::GetFileSystem)?;

    let mut root = fs.open_volume().map_err(|_| Error::RootVolume)?;

    let profiles: Vec<Profile> = Vec::new();

    Ok(profiles)
}


