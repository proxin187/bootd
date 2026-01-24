use crate::error::Error;

use core::mem::MaybeUninit;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;

use uefi::proto::media::file::{File, FileMode, FileAttribute, FileInfo, Directory};
use uefi::proto::device_path::build::media::FilePath;
use uefi::proto::device_path::build::DevicePathBuilder;
use uefi::boot::{self, LoadImageSource};
use uefi::proto::BootPolicy;
use uefi::fs::PathBuf;
use uefi::CString16;


pub struct Profile {
    pub name: String,
    pub kernel: CString16,
    pub cmdline: String,
}

impl Profile {
    pub fn new(root: &mut Directory, info: &FileInfo, path: PathBuf) -> Result<Profile, Error> {
        let mut handle = root.open(path.to_cstr16(), FileMode::Read, FileAttribute::empty()).ok()
            .and_then(|file| file.into_regular_file())
            .ok_or_else(|| Error::InvalidProfilesDirectory(path.clone()))?;

        let mut buffer: Vec<u8> = vec![0; info.file_size() as usize];

        handle.read(&mut buffer).map_err(|_| Error::ReadEntryFailed)?;

        let mut lines = buffer.split(|character| *character as char == '\n')
            .map(|line| String::from_utf8_lossy(line));

        Ok(Profile {
            name: lines.next().ok_or(Error::InvalidProfileFormat)?.to_string(),
            kernel: lines.next().and_then(|line| CString16::try_from(line.as_ref()).ok()).ok_or(Error::InvalidProfileFormat)?,
            cmdline: lines.next().ok_or(Error::InvalidProfileFormat)?.to_string(),
        })
    }

    pub fn boot_image(&self) -> Result<(), Error> {
        let mut buffer = [MaybeUninit::uninit(); 256];

        let path = DevicePathBuilder::with_buf(&mut buffer)
            .push(&FilePath {
                path_name: &self.kernel,
            })
            .expect("unreachable")
            .finalize()
            .map_err(|_| Error::DevicePathFailed)?;

        let handle = boot::load_image(boot::image_handle(), LoadImageSource::FromDevicePath { device_path: path, boot_policy: BootPolicy::BootSelection })
            .map_err(|_| Error::LoadImageFailed)?;

        // TODO: we need to set the kernel command line parameters and then start the image

        Ok(())
    }
}

pub fn load_profiles(path: PathBuf) -> Result<Vec<Profile>, Error> {
    let mut fs = boot::get_image_file_system(boot::image_handle())
        .map_err(|_| Error::GetFileSystem)?;

    let mut root = fs.open_volume().map_err(|_| Error::RootVolume)?;

    let mut dir = root.open(path.to_cstr16(), FileMode::Read, FileAttribute::empty()).ok()
        .and_then(|dir| dir.into_directory())
        .ok_or_else(|| Error::InvalidProfilesDirectory(path.clone()))?;

    let mut profiles: Vec<Profile> = Vec::new();

    while let Some(entry) = dir.read_entry_boxed().map_err(|_| Error::ReadEntryFailed)? {
        if entry.is_regular_file() {
            let mut entry_path = path.clone();

            entry_path.push(entry.file_name());

            profiles.push(Profile::new(&mut root, entry.as_ref(), entry_path)?);
        }
    }

    Ok(profiles)
}


