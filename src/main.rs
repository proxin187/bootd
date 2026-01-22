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

    let mut menu = Menu::new(profiles)?;

    let profile = menu.select()?;

    println!("info: booting: {}", profile.name);

    Ok(())
}

#[entry]
fn entry() -> Status {
    if let Err(err) = main() {
        panic!("{}", err);
    }

    Status::SUCCESS
}


