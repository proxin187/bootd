#![no_std]
#![no_main]

extern crate alloc;

mod profile;
mod error;
mod options;

use options::Options;

use uefi::prelude::*;
use uefi::println;


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("error: bootd panic: {}", info.message());

    loop {}
}

#[entry]
fn main() -> Status {
    // TODO: remove these unwraps, this is only to test that Options works, proper error handling
    // is next
    let options = Options::new().unwrap();

    println!("info: bootd.profiles={}", options.profiles().unwrap());

    loop {}

    Status::SUCCESS
}


