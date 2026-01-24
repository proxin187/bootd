use crate::profile::Profile;
use crate::error::Error;

use alloc::vec::Vec;

use uefi::proto::console::text::{Key, ScanCode};
use uefi::{println, boot, system};


pub struct Menu {
    select: usize,
    profiles: Vec<Profile>,
}

impl Menu {
    pub fn new(profiles: Vec<Profile>) -> Result<Menu, Error> {
        if !profiles.is_empty() {
            Ok(Menu {
                select: 0,
                profiles,
            })
        } else {
            Err(Error::NoProfiles)
        }
    }

    fn disable_cursor(&self) {
        system::with_stdout(|output| {
            let _ = output.enable_cursor(false);
        });
    }

    fn wait_for_input(&mut self) -> Result<Option<Key>, Error> {
        system::with_stdin(|input| {
            boot::wait_for_event(&mut [input.wait_for_key_event().unwrap()]).map_err(|_| Error::InputFailed)?;

            input.read_key().map_err(|_| Error::InputFailed)
        })
    }

    pub fn select<'a>(&'a mut self) -> Result<&'a Profile, Error> {
        self.disable_cursor();

        println!("info: available profiles:");

        loop {
            for (index, profile) in self.profiles.iter_mut().enumerate() {
                if index == self.select {
                    println!("> {}", profile.name);
                } else {
                    println!("{}  ", profile.name);
                }
            }

            match self.wait_for_input()? {
                Some(Key::Special(ScanCode::UP)) => self.select = self.select.max(1) - 1,
                Some(Key::Special(ScanCode::DOWN)) => self.select = self.profiles.len().min(self.select + 2) - 1,
                Some(Key::Printable(character)) if Into::<char>::into(character) == '\r' => return Ok(&self.profiles[self.select]),
                _ => {},
            }

            system::with_stdout(|output| -> Result<(), Error> {
                let (_, row) = output.cursor_position();

                output.set_cursor_position(0, row - self.profiles.len()).map_err(|_| Error::OutputFailed)
            })?;
        }
    }
}



