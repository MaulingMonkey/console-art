use maulingmonkey_console_art::*;

fn main() {
    #[cfg(windows)] {
        use maulingmonkey_console_winapi_wrappers::*;
        let _ = change_console_mode(&mut std::io::stdout(), |m| m | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
    }
    let image = Image::try_from_png(std::io::Cursor::new(include_bytes!("chalkboard.png"))).unwrap();
    print!("{}", image.display_basic_24bpp_halfblocks(0));
}
