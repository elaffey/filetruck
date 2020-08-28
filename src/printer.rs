use std::io::Write;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub struct Printer {
    buffer_writer: BufferWriter,
    buffer: Buffer,
}

pub trait Print {
    fn writeln(&mut self, s: impl std::fmt::Display);
    fn print(&self);
}

impl Printer {
    #[must_use]
    pub fn stdout(color_choice: ColorChoice) -> Self {
        let buffer_writer = BufferWriter::stdout(color_choice);
        let mut buffer = buffer_writer.buffer();
        let color_spec = ColorSpec::new();
        if let Err(e) = buffer.set_color(&color_spec) {
            eprintln!("Could not set color for stdout - {}", e);
        }
        Self {
            buffer_writer,
            buffer,
        }
    }

    #[must_use]
    pub fn stderr(color_choice: ColorChoice) -> Self {
        let buffer_writer = BufferWriter::stderr(color_choice);
        let mut buffer = buffer_writer.buffer();
        let mut color_spec = ColorSpec::new();
        color_spec
            .set_fg(Some(Color::Red))
            .set_intense(true)
            .set_bold(true);
        if let Err(e) = buffer.set_color(&color_spec) {
            eprintln!("Could not set color for stderr - {}", e);
        }
        Self {
            buffer_writer,
            buffer,
        }
    }
}

impl Print for Printer {
    fn writeln(&mut self, s: impl std::fmt::Display) {
        if let Err(e) = writeln!(self.buffer, "{}", s) {
            eprintln!("Could not write to buffer - {}", e)
        }
    }

    fn print(&self) {
        if let Err(e) = self.buffer_writer.print(&self.buffer) {
            eprintln!("Could not print buffer - {}", e)
        }
    }
}

impl Drop for Printer {
    fn drop(&mut self) {
        self.print();
    }
}
