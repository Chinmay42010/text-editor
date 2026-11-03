use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::{queue, Command};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use core::fmt::Display;
use std::io::{stdout, Error, Write};

#[derive(Copy, Clone)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}
pub struct Position {
    pub x: u16,
    pub y: u16,
}
pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position {x:0, y:0})?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_cursor_to(position: Position) -> Result<(), std::io::Error> {
        Self::queue_command(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(stdout(), Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(stdout(), Show)?;
        Ok(())
    }
    pub fn print<T: Display>(string: T) -> Result<(), Error> {
        Self::queue_command(stdout(), Print(string))?;
        Ok(())
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
    pub fn size() -> Result<Size, Error> {
    let (width, height) = crossterm::terminal::size()?;

    Ok(Size { height, width })
    }
    fn queue_command<T: Command>(mut writer: impl Write, command: T) -> Result<(), Error> {
        queue!(writer, command)?;
        Ok(())
    }
}
