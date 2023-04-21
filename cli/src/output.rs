use spinners::{Spinner, Spinners};
use std::{thread, time};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct NautilusTerminal {
    // spinner: Spinner,
    stdout: StandardStream,
}

impl NautilusTerminal {
    pub fn create_spinner(msg: &str) -> Spinner {
        println!("\n\n");
        let mut sp = Spinner::new(Spinners::CircleQuarters, msg.into());
        thread::sleep(time::Duration::from_secs(2));
        sp.stop(); // May remove, stopped intentionally for now
        println!("\n\n");
        sp
    }

    pub fn new(color: Color, msg: &str) -> Self {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let mut colspec = ColorSpec::new();
        colspec.set_fg(Some(color)).set_bold(true);
        stdout.set_color(&colspec).unwrap();
        println!("\n-----------------------------------------");
        let _spinner = Self::create_spinner(msg);
        NautilusTerminal { stdout }
    }

    pub fn output(&mut self, color: Color, msg: &str) {
        let mut colspec = ColorSpec::new();
        colspec.set_fg(Some(color)).set_bold(true);
        self.stdout.set_color(&colspec).unwrap();
        println!("\n\n{}", msg);
    }

    pub fn end_output(&mut self, color: Color, msg: &str) {
        let mut colspec = ColorSpec::new();
        colspec.set_fg(Some(color)).set_bold(true);
        self.stdout.set_color(&colspec).unwrap();
        println!("\n\n{}", msg);
        println!("\n-----------------------------------------");
        self.stdout.reset().unwrap();
    }
}
