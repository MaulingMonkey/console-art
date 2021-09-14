use std::io::Write;

use maulingmonkey_console_art::*;
use maulingmonkey_console_escape_codes::vt100::*;

fn main() {
    let _art = ArtMode::enable();
    eprintln!("window: {:?}", console_window_wh());
    print!("{}", alternate_screen_buffer());
    let _ = std::io::stdout().flush();
    let wh = console_window_wh();
    print!("{}", main_screen_buffer());
    let _ = std::io::stdout().flush();
    eprintln!("window: {:?}", wh);
    eprintln!("window: {:?}", console_window_wh());
}
