mod console;

use console::Console;

use crate::profile::Profile;
use crate::error::Error;

use alloc::format;

use uefi::proto::console::text::{Key, ScanCode, Color, Input, Output};


pub struct Menu<'a> {
    console: Console<'a>,
    profiles: &'a [Profile],
    current: usize,
}

impl<'a> Menu<'a> {
    pub fn new(output: &'a mut Output, input: &'a mut Input, profiles: &'a [Profile]) -> Result<Menu<'a>, Error> {
        if !profiles.is_empty() {
            Ok(Menu {
                console: Console::new(output, input)?,
                profiles,
                current: 0,
            })
        } else {
            Err(Error::NoProfiles)
        }
    }

    fn initialize(&mut self) -> Result<(), Error> {
        let columns = self.console.columns();
        let rows = self.console.rows();

        let header = format!("bootd v{}", env!("CARGO_PKG_VERSION"));
        let footer = "[Up/Down: Select] [Enter: Boot]";

        self.console.write_at(&header, |_| (columns / 2) - (header.len() / 2), |_| 0)?;

        self.console.write_at(&footer, |_| (columns / 2) - (footer.len() / 2), |_| rows - 1)?;

        let rows = self.console.rows();

        self.console.move_cursor(|_| 0, |_| (rows / 2) - (self.profiles.len() / 2))
    }

    pub fn select(&mut self) -> Result<usize, Error> {
        self.console.clear()?;

        self.initialize()?;

        loop {
            for (index, profile) in self.profiles.iter().enumerate() {
                if index == self.current {
                    self.console.set_color(Color::Black, Color::LightGray)?;
                } else {
                    self.console.set_color(Color::White, Color::Black)?;
                }

                self.console.write_profile(&profile)?;
            }

            match self.console.wait_for_input()? {
                Some(Key::Special(ScanCode::UP)) => self.current = self.current.max(1) - 1,
                Some(Key::Special(ScanCode::DOWN)) => self.current = self.profiles.len().min(self.current + 2) - 1,
                Some(Key::Printable(character)) if Into::<char>::into(character) == '\r' => {
                    self.console.set_color(Color::White, Color::Black)?;

                    self.console.clear()?;

                    return Ok(self.current);
                },
                _ => {},
            }

            self.console.move_cursor(|column| column, |row| row - self.profiles.len())?;
        }
    }
}


