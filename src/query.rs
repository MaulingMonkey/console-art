use std::io;



pub fn console_buffer_wh() -> io::Result<(usize, usize)> { imp::console_buffer_wh() }

pub fn console_buffer_width() -> io::Result<usize> { Ok(console_buffer_wh()?.0) }
pub fn console_buffer_height() -> io::Result<usize> { Ok(console_buffer_wh()?.1) }



#[cfg(windows)] mod imp {
    use super::*;
    use maulingmonkey_console_winapi_wrappers::*;

    pub fn console_buffer_wh() -> io::Result<(usize, usize)> {
        let info = get_console_screen_buffer_info(&std::io::stdout())?;
        Ok((info.dwSize.X as _, info.dwSize.Y as _))
    }

    #[allow(dead_code)]
    pub fn console_cursor_xy() -> io::Result<(usize, usize)> {
        let info = get_console_screen_buffer_info(&std::io::stdout())?;
        Ok((info.dwCursorPosition.X as _, info.dwCursorPosition.Y as _))
    }
}

#[cfg(unix)] mod imp {
    use super::*;
    use libc::*;
    use std::mem::zeroed;
    use std::os::unix::prelude::*;

    pub fn console_buffer_wh() -> io::Result<(usize, usize)> {
        let mut size : winsize = unsafe { zeroed() };
        match unsafe { ioctl(std::io::stdout().as_raw_fd(), TIOCGWINSZ, &mut size) } {
            0 => {},
            n => return Err(io::Error::from_raw_os_error(n)),
        }
        Ok((size.ws_col as _, size.ws_row as _))
    }
}
