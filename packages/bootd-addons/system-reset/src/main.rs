#![no_std]
#![no_main]

extern crate alloc;

mod error;
mod reset;

use reset::Reset;
use error::Error;

use bootd_common::options::Options;
use uefi::prelude::*;


#[cfg(not(test))]
use uefi::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("error: system-reset panic: {}", info.message());

    loop {}
}

fn main() -> Result<(), Error> {
    let options = Options::new()
        .map_err(|err| Error::Uefi(err))
        .and_then(|options| options.ok_or(Error::NoOptions))?;

    let reset = Reset::try_from(options.arg("--reset-type ").ok_or(Error::NoResetType)?)?;

    reset.reset()
}

#[entry]
fn entry() -> Status {
    if let Err(err) = main() {
        panic!("{}", err);
    }

    Status::SUCCESS
}


