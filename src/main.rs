#![no_std]
#![no_main]

extern crate alloc;

mod profile;
mod options;
mod error;
mod menu;

use options::Options;
use error::Error;
use menu::Menu;

use uefi::prelude::*;
use uefi::println;


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("error: bootd panic: {}", info.message());

    loop {}
}

fn main() -> Result<(), Error> {
    let options = Options::new()?;

    let path = options.profiles()?;

    println!("info: bootd.profiles={}", path);

    let profiles = profile::load_profiles(path)?;

    let profile = system::with_stdout(|output| -> Result<usize, Error> {
        system::with_stdin(|input| -> Result<usize, Error> {
            let mut menu = Menu::new(output, input, &profiles)?;

            menu.select()
        })
    })?;

    println!("info: booting: {}", profiles[profile].name);

    profiles[profile].boot_image()
}

#[entry]
fn entry() -> Status {
    if let Err(err) = main() {
        panic!("{}", err);
    }

    Status::SUCCESS
}


