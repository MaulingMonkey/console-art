use maulingmonkey_console_escape_codes::*;
use maulingmonkey_console_escape_codes::vt100::*;

use std::fmt::Display;



pub fn spaces(left: impl Into<Color>, right: impl Into<Color>, mid: usize, width: usize) -> impl Display {
    let mid = mid.min(width);
    format!(
        "{left}{empty: >mid$}{right}{empty: >width_mid$}{reset}",
        left        = vt100::sgr_background_color(left),
        right       = vt100::sgr_background_color(right),
        empty       = "",
        mid         = mid,
        width_mid   = width-mid,
        reset       = vt100::sgr_background_default(),
    )
}

pub fn eighth_blocks(left: impl Into<Color>, right: impl Into<Color>, mid8: usize, width8: usize) -> impl Display {
    if width8 == 0 { return String::new(); }

    let mid8            = mid8.min(width8);
    let total_blocks    = (width8+7)/8;
    let full_blocks     = mid8/8;
    let mid_ch          = ["", "\u{258F}", "\u{258E}", "\u{258D}", "\u{258C}", "\u{258B}", "\u{258A}", "\u{2589}"][mid8 & 7];
    let mid_blocks      = usize::from(!mid_ch.is_empty());
    let empty_blocks    = total_blocks - full_blocks - mid_blocks;

    format!(
        "{left}{right}{empty:\u{2588}>full_blocks$}{mid_ch}{empty: >empty_blocks$}{reset_fg}{reset_bg}",
        left            = vt100::sgr_foreground_color(left),
        right           = vt100::sgr_background_color(right),
        empty           = "",
        mid_ch          = mid_ch,
        full_blocks     = full_blocks,
        empty_blocks    = empty_blocks,
        reset_fg        = vt100::sgr_foreground_default(),
        reset_bg        = vt100::sgr_foreground_default(),
    )
}

pub fn text(left: impl Into<Style>, right: impl Into<Style>, mid: usize, n: usize, text: impl AsRef<str>) -> impl Display {
    let left    = left.into();
    let right   = right.into();
    let text    = text.as_ref();
    let mid     = mid.min(n);


    let (mid, n) = if n.checked_mul(text.len()).is_some() {
        (mid, n)
    } else {
        (mid / text.len(), n / text.len())
    };

    let mid_len = mid * text.len();

    let mid = text.char_indices().find(|(idx, _ch)| idx * n >= mid_len).map(|(idx, _ch)| idx).unwrap_or(text.len());
    let left_text = &text[0 .. mid];
    let right_text = &text[mid ..];
    format!(
        "{left_style}{left_text}{right_style}{right_text}{reset}",
        left_style  = left,
        left_text   = left_text,
        right_style = right,
        right_text  = right_text,
        reset       = vt100::sgr_default(),
    )
}
