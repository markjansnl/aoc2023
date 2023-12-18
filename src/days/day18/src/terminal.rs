use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{PrintStyledContent, Stylize},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use super::{Command, Executor};
use crate::prelude::*;

pub struct TerminalPainter {
    stdout: Stdout,
}

impl Default for TerminalPainter {
    fn default() -> Self {
        let mut stdout = stdout();
        stdout
            .execute(EnterAlternateScreen)
            .context("Cannot enter alternate screen")
            .unwrap();
        Self { stdout }
    }
}

impl Drop for TerminalPainter {
    fn drop(&mut self) {
        self.stdout
            .execute(LeaveAlternateScreen)
            .context("Cannot enter alternate screen")
            .unwrap();
    }
}

impl Executor for TerminalPainter {
    fn execute(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Dig(position) => {
                if position.x >= 0 && position.y >= 0 && position.x < 100 && position.y < 50 {
                    self.stdout
                        .execute(MoveTo(position.x as u16, position.y as u16))?;
                    self.stdout.execute(PrintStyledContent("#".grey()))?;
                }
            }
            Command::Paint(position, _rgb) => {
                if position.x >= 0 && position.y >= 0 && position.x < 100 && position.y < 50 {
                    self.stdout
                        .execute(MoveTo(position.x as u16, position.y as u16))?;
                    self.stdout.execute(PrintStyledContent("#".green()))?;
                }
            }
        }
        self.stdout.flush()?;
        Ok(())
    }
}
