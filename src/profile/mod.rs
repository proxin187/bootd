use crate::error::Error;

use core::mem::MaybeUninit;

use alloc::string::{String, ToString};
use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::vec;

use uefi::proto::media::file::{File, FileMode, FileAttribute, FileInfo, Directory};
use uefi::proto::device_path::text::{DisplayOnly, AllowShortcuts};
use uefi::proto::device_path::build::media::FilePath;
use uefi::proto::device_path::build::DevicePathBuilder;
use uefi::proto::device_path::DevicePath;
use uefi::proto::loaded_image::LoadedImage;
use uefi::boot::{self, LoadImageSource, ScopedProtocol};
use uefi::proto::BootPolicy;
use uefi::fs::PathBuf;
use uefi::{println, CString16};


pub struct Profile {
    pub name: String,
    pub kernel: CString16,
    pub cmdline: Vec<u16>,
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

        let name = lines.next().ok_or(Error::InvalidProfileFormat)?.to_string();
        let kernel = lines.next().and_then(|line| CString16::try_from(line.as_ref()).ok()).ok_or(Error::InvalidProfileFormat)?;
        let cmdline = lines.next().ok_or(Error::InvalidProfileFormat)?.to_string();

        Ok(Profile {
            name,
            kernel,
            cmdline: cmdline.encode_utf16().collect(),
        })
    }

    fn device_path(&self) -> Result<ScopedProtocol<DevicePath>, Error> {
        let loaded_image = boot::open_protocol_exclusive::<LoadedImage>(boot::image_handle())
            .map_err(|err| Error::OpenProtocol(Box::new(err)))?;

        let device = loaded_image.device().ok_or(Error::DevicePathFailed)?;

        boot::open_protocol_exclusive::<DevicePath>(device)
            .map_err(|err| Error::OpenProtocol(Box::new(err)))
    }

    pub fn boot_image(&self) -> Result<(), Error> {
        let mut buffer = [MaybeUninit::uninit(); 256];

        let mut builder = DevicePathBuilder::with_buf(&mut buffer);

        for node in self.device_path()?.node_iter() {
            builder = builder.push(&node).map_err(|_| Error::DevicePathFailed)?;
        }

        let path = builder.push(&FilePath { path_name: &self.kernel })
            .map_err(|_| Error::DevicePathFailed)?
            .finalize()
            .map_err(|_| Error::DevicePathFailed)?;

        if let Ok(path_str) = path.to_string(DisplayOnly(true), AllowShortcuts(true)) {
            println!("info: device path: {}", path_str);
        }

        let handle = boot::load_image(boot::image_handle(), LoadImageSource::FromDevicePath { device_path: path, boot_policy: BootPolicy::BootSelection })
            .map_err(|_| Error::LoadImageFailed(self.kernel.clone()))?;

        let mut loaded_image = boot::open_protocol_exclusive::<LoadedImage>(handle)
            .map_err(|err| Error::OpenProtocol(Box::new(err)))?;

        unsafe {
            loaded_image.set_load_options(self.cmdline.as_ptr() as *const u8, (self.cmdline.len() * 2) as u32);
        }

        boot::start_image(handle).map_err(|_| Error::StartImageFailed)
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


