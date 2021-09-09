use maulingmonkey_console_art::*;
use maulingmonkey_console_escape_codes::*;
use maulingmonkey_console_escape_codes::vt100::*;

use crate::sliders;

fn main() {
    #[cfg(windows)] {
        use maulingmonkey_console_winapi_wrappers::*;
        let _ = change_console_mode(&mut std::io::stdout(), |m| m | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
    }
    print!("{}", alternate_screen_buffer());

    let max = 121; // 120 = divisible by eight
    for n in  0 ..= max         { progress_demo(n, max); }
    for n in (0 ..= max).rev()  { progress_demo(n, max); }

    print!("{}", main_screen_buffer());
}

fn progress_demo(n: usize, max: usize) {
    let left_c = Palette::GREEN;
    let right_c = Color::default();
    let max_mul = usize::MAX / max;

    let left = Style {
        foreground: Some(Palette::BLACK.into()),
        background: Some(Palette::GREEN.into()),
        .. Default::default()
    };
    let right = Style {
        foreground: Some(Palette::GREEN.into()),
        background: Some(Palette::BLACK.into()),
        .. Default::default()
    };

    print!("{}", cursor_position(RowColNo(1,1)));
    println!();
    println!();
    println!("{}", sliders::spaces(left_c.clone(), right_c.clone(), n, max-0));
    println!("{}", sliders::spaces(left_c.clone(), right_c.clone(), n, max-1));
    println!("{}", sliders::spaces(left_c.clone(), right_c.clone(), n, max-2));
    println!();
    println!();
    println!("{}", sliders::eighth_blocks(left_c.clone(), right_c.clone(), n, max-0));
    println!("{}", sliders::eighth_blocks(left_c.clone(), right_c.clone(), n, max-1));
    println!("{}", sliders::eighth_blocks(left_c.clone(), right_c.clone(), n, max-2));
    println!();
    println!();
    println!("{}", sliders::text(left.clone(), right.clone(), n, max-0, "  this is a progress bar  "));
    println!("{}", sliders::text(left.clone(), right.clone(), n * max_mul, max * max_mul, "  this is a progress bar  "));
    println!();
    println!();
    std::thread::sleep(std::time::Duration::from_millis(16));
}
