use crate::profile::Profile;
use crate::error::Error;

use core::fmt::Write;

use uefi::proto::console::text::{Key, ScanCode, Color, Input, Output, OutputMode};
use uefi::boot;


pub struct Console<'a> {
    output: &'a mut Output,
    input: &'a mut Input,
    mode: OutputMode,
}

impl<'a> Console<'a> {
    pub fn new(output: &'a mut Output, input: &'a mut Input) -> Result<Console<'a>, Error> {
        let mode = output.current_mode()
            .map_err(|err| Error::OutputFailed(Some(err)))?
            .ok_or(Error::NoMode)?;

        Ok(Console {
            output,
            input,
            mode,
        })
    }

    pub fn wait_for_input(&mut self) -> Result<Option<Key>, Error> {
        boot::wait_for_event(&mut [self.input.wait_for_key_event().unwrap()]).map_err(|err| Error::InputFailed(Some(err.to_err_without_payload())))?;

        self.input.read_key().map_err(|err| Error::InputFailed(Some(err)))
    }

    pub fn clear(&mut self, profiles: usize) -> Result<(), Error> {
        // TODO: this errors in qemu with edk 2 ovmf, it should work on real hardware though
        // self.output.enable_cursor(false).map_err(|err| Error::OutputFailed(Some(err)))?;
        self.output.clear().map_err(|err| Error::OutputFailed(Some(err)))?;

        let columns = self.mode.columns();
        let rows = self.mode.rows();

        self.move_cursor(|_| columns / 2, |_| (rows / 2) - (profiles / 2))
    }

    pub fn set_color(&mut self, foreground: Color, background: Color) -> Result<(), Error> {
        self.output.set_color(foreground, background).map_err(|err| Error::OutputFailed(Some(err)))
    }

    pub fn move_cursor<C: Fn(usize) -> usize, R: Fn(usize) -> usize>(&mut self, column: C, row: R) -> Result<(), Error> {
        let (column_position, row_position) = self.output.cursor_position();

        self.output.set_cursor_position(column(column_position), row(row_position)).map_err(|err| Error::OutputFailed(Some(err)))
    }

    pub fn write_profile(&mut self, profile: &Profile) -> Result<(), Error> {
        let columns = self.mode.columns();

        self.output.write_str(&profile.name).map_err(|_| Error::OutputFailed(None))?;

        self.move_cursor(|_| columns / 2, |row| row + 1)
    }
}

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

    pub fn select(&mut self) -> Result<usize, Error> {
        self.console.clear(self.profiles.len())?;

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

                    return Ok(self.current);
                },
                _ => {},
            }

            self.console.move_cursor(|column| column, |row| row - self.profiles.len())?;
        }
    }
}



