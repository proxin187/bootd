use crate::profile::Profile;
use crate::error::Error;

use core::fmt::Write;

use uefi::proto::console::text::{Key, Color, Input, Output, OutputMode};
use uefi::boot;



pub struct Console<'a> {
    output: &'a mut Output,
    input: &'a mut Input,
    mode: OutputMode,
}

impl<'a> Console<'a> {
    pub fn new(output: &'a mut Output, input: &'a mut Input) -> Result<Console<'a>, Error> {
        let mode = output.current_mode()?.ok_or(Error::NoMode)?;

        Ok(Console {
            output,
            input,
            mode,
        })
    }

    #[inline(always)]
    pub fn rows(&self) -> usize { self.mode.rows() }

    #[inline(always)]
    pub fn columns(&self) -> usize { self.mode.columns() }

    pub fn wait_for_input(&mut self) -> Result<Option<Key>, Error> {
        boot::wait_for_event(&mut [self.input.wait_for_key_event().unwrap()])?;

        Ok(self.input.read_key()?)
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        // TODO: this errors in qemu with edk 2 ovmf, it should work on real hardware though
        // self.output.enable_cursor(false)?;

        Ok(self.output.clear()?)
    }

    pub fn set_color(&mut self, foreground: Color, background: Color) -> Result<(), Error> {
        Ok(self.output.set_color(foreground, background)?)
    }

    pub fn move_cursor<C: Fn(usize) -> usize, R: Fn(usize) -> usize>(&mut self, column: C, row: R) -> Result<(), Error> {
        let (column_position, row_position) = self.output.cursor_position();

        Ok(self.output.set_cursor_position(column(column_position), row(row_position))?)
    }

    pub fn write_str(&mut self, string: impl AsRef<str>) -> Result<(), Error> {
        Ok(self.output.write_str(string.as_ref())?)
    }

    pub fn write_at<C: Fn(usize) -> usize, R: Fn(usize) -> usize>(&mut self, string: impl AsRef<str>, column: C, row: R) -> Result<(), Error> {
        self.move_cursor(column, row)?;

        self.write_str(string)
    }

    pub fn write_profile(&mut self, profile: &Profile) -> Result<(), Error> {
        let columns = self.mode.columns();

        self.write_at(&profile.name, |_| (columns / 2) - (profile.name.len() / 2), |row| row)?;

        self.move_cursor(|_| 0, |row| row + 1)
    }
}


