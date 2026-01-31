#![no_std]
#![no_main]

extern crate alloc;

mod profile;
mod error;
mod menu;

use error::Error;
use menu::Menu;

use bootd_common::options::Options;
use uefi::prelude::*;
use uefi::fs::PathBuf;
use uefi::CString16;

#[cfg(not(test))]
use uefi::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("error: bootd panic: {}", info.message());

    loop {}
}

fn main() -> Result<(), Error> {
    let options = Options::new()
        .map_err(|err| Error::Uefi(err))
        .and_then(|options| options.ok_or(Error::NoOptions))?;

    let path = options.arg("--profiles ")
        .ok_or(Error::NoProfilesPath)
        .and_then(|path| CString16::try_from(path).map_err(|_| Error::NoProfilesPath))
        .map(|path| PathBuf::from(path))?;

    let profiles = profile::load_profiles(path)?;

    let profile = system::with_stdout(|output| -> Result<usize, Error> {
        system::with_stdin(|input| -> Result<usize, Error> {
            let mut menu = Menu::new(output, input, &profiles)?;

            menu.select()
        })
    })?;

    profiles[profile].launch_image()
}

#[entry]
fn entry() -> Status {
    if let Err(err) = main() {
        panic!("{}", err);
    }

    Status::SUCCESS
}


