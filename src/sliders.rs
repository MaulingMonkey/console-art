use crate::*;

use maulingmonkey_console_escape_codes::*;
use maulingmonkey_console_escape_codes::vt100::*;

use std::fmt::Display;






pub fn spaces(left: impl Into<Color>, right: impl Into<Color>, mid: impl Ratio, width: usize) -> impl Display {
    let mid = mid.to_0n_clamped(width);
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

pub fn eighth_blocks(left: impl Into<Color>, right: impl Into<Color>, mid: impl Ratio, width: usize) -> impl Display {
    if width == 0 { return String::new(); }

    let width8          = width * 8;
    let mid8            = mid.to_0n_clamped(width8);
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

pub fn text(left: impl Into<Style>, right: impl Into<Style>, mid: impl Ratio, text: impl AsRef<str>) -> impl Display {
    let left    = left.into();
    let right   = right.into();
    let text    = text.as_ref();
    let mid     = mid.to_0n_clamped(text.len());
    let mid     = text.char_indices().find(|&(idx, _ch)| idx >= mid).map(|(idx, _ch)| idx).unwrap_or(text.len()); // round mid up to character bound
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
