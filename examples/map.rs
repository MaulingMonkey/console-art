use maulingmonkey_console_art::*;

fn main() {
    let _art = ArtMode::enable();
    let image = Image::try_from_png(std::io::Cursor::new(include_bytes!("map.png"))).unwrap();
    print!("{}", image.display_basic_24bpp_halfblocks(0));
}
