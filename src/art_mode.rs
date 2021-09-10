#[cfg(unix)]    use termios::*;
#[cfg(unix)]    use std::os::unix::prelude::*;
#[cfg(windows)] use maulingmonkey_console_winapi_wrappers::*;

use std::io;



pub struct ArtMode {
    #[cfg(unix)]    original_termios:       Termios,
    #[cfg(windows)] original_input_mode:    InputMode,
    #[cfg(windows)] original_output_mode:   OutputMode,
}

impl ArtMode {
    pub fn enable() -> io::Result<Self> { Self::enable_impl() }
    pub fn disable(self) {}
}

impl Drop for ArtMode {
    fn drop(&mut self) {
        let _ = self.disable_impl();
    }
}



#[cfg(windows)] impl ArtMode {
    fn enable_impl() -> io::Result<Self> {
        let mode = Self {
            original_input_mode:    get_console_mode(&mut std::io::stdin())?,
            original_output_mode:   get_console_mode(&mut std::io::stdout())?,
        };

        set_console_mode(&mut std::io::stdin(),  ENABLE_MOUSE_INPUT | ENABLE_PROCESSED_INPUT | ENABLE_WINDOW_INPUT | ENABLE_VIRTUAL_TERMINAL_INPUT)?;
        set_console_mode(&mut std::io::stdout(), ENABLE_PROCESSED_OUTPUT | ENABLE_VIRTUAL_TERMINAL_PROCESSING)?;
        Ok(mode)
    }

    fn disable_impl(&mut self) -> io::Result<()> {
        set_console_mode(&mut std::io::stdin(),  self.original_input_mode)?;
        set_console_mode(&mut std::io::stdout(), self.original_output_mode)?;
        Ok(())
    }
}

#[cfg(unix)] impl ArtMode {
    fn enable_impl() -> io::Result<Self> {
        let mode = Self {
            original_termios: Termios::from_fd(std::io::stdout().as_raw_fd())?
        };
        let mut new_termios = mode.original_termios;
        cfmakeraw(&mut new_termios);
        new_termios.c_oflag |= OPOST; // Keep translation of "\n" => "\r\n"
        tcsetattr(std::io::stdout().as_raw_fd(), TCSADRAIN, &new_termios)?;
        Ok(mode)
    }

    fn disable_impl(&mut self) -> io::Result<()> {
        tcsetattr(std::io::stdout().as_raw_fd(), TCSADRAIN, &self.original_termios)?;
        Ok(())
    }
}
