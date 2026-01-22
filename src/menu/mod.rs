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

    fn wait_for_input(&mut self) -> Result<Option<Key>, Error> {
        system::with_stdin(|input| {
            boot::wait_for_event(&mut [input.wait_for_key_event().unwrap()]).map_err(|_| Error::InputFailed)?;

            input.read_key().map_err(|_| Error::InputFailed)
        })
    }

    pub fn select<'a>(&'a mut self) -> Result<&'a Profile, Error> {
        loop {
            system::with_stdout(|output| -> Result<(), Error> {
                output.clear().map_err(|_| Error::OutputFailed)
            })?;

            for (index, profile) in self.profiles.iter().enumerate() {
                if index == self.select {
                    println!("> {} - {}", profile.name, profile.kernel);
                } else {
                    println!("{} - {}", profile.name, profile.kernel);
                }
            }

            match self.wait_for_input()? {
                Some(Key::Special(ScanCode::UP)) => self.select = self.select.max(1) - 1,
                Some(Key::Special(ScanCode::DOWN)) => self.select = self.profiles.len().min(self.select + 2) - 1,
                Some(Key::Printable(character)) if Into::<char>::into(character) == '\r' => return Ok(&self.profiles[self.select]),
                _ => {},
            }
        }
    }
}



