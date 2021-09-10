use crate::*;
use maulingmonkey_console_escape_codes::{RGB, vt100};
use std::convert::*;
use std::fmt::{self, Display, Formatter};



pub struct Image {
    data:   Vec<RGB>,
    width:  usize,
    height: usize,
}

impl Image {
    pub fn size(&self) -> (usize, usize) { (self.width, self.height) }
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn get(&self, xy: impl TryIntoXY) -> Option<RGB> {
        let (x, y) = xy.try_into_xy()?;
        if x >= self.width  { return None; }
        if y >= self.height { return None; }
        self.data.get(y * self.width + x).copied()
    }

    pub fn get_mut(&mut self, xy: impl TryIntoXY) -> Option<&mut RGB> {
        let (x, y) = xy.try_into_xy()?;
        if x >= self.width  { return None; }
        if y >= self.height { return None; }
        self.data.get_mut(y * self.width + x)
    }

    /// Use [24-bit Color ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code#24-bit)
    /// and [`"▀▄"` Upper/Lower half blocks](https://en.wikipedia.org/wiki/Block_Elements)
    /// to render images at a resolution of 2 pixels per console character.
    pub fn display_basic_24bpp_halfblocks(&self, pad_left: usize) -> impl Display + '_ { DisplayVT100Basic { pad_left, image: self } }
}



struct DisplayVT100Basic<'a> {
    pad_left:   usize,
    image:      &'a Image,
}

impl Display for DisplayVT100Basic<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", vt100::sgr_default())?;
        for y0 in (0 .. self.image.height).into_iter().step_by(2) {
            if self.pad_left > 0 {
                write!(f, "{: >width$}", "", width=self.pad_left)?;
            }
            let y1 = y0 + 1;
            for x in 0 .. self.image.width {
                let t = self.image.get((x,y0));
                let b = self.image.get((x,y1));

                // https://en.wikipedia.org/wiki/Block_Elements
                match (t, b) {
                    (None, None)        => write!(f, " ")?,
                    (Some(t), None)     => write!(f, "{}\u{2580}", vt100::sgr_foreground_rgb(t))?, // U+2580 Upper half block
                    (None, Some(b))     => write!(f, "{}\u{2584}", vt100::sgr_foreground_rgb(b))?, // U+2584 Lower half block
                    (Some(t), Some(b))  => write!(f, "{}{}\u{2580}", vt100::sgr_foreground_rgb(t), vt100::sgr_background_rgb(b))?,
                }
            }
            write!(f, "{}\r\n", vt100::sgr_default())?;
        }
        Ok(())
    }
}



#[cfg(feature = "png")] mod _png {
    use super::*;
    use png::*;
    use std::io::{self, ErrorKind::InvalidData, Read};
    use std::path::Path;

    impl Image {
        pub fn try_from_png_path(path: impl AsRef<Path>) -> io::Result<Self> {
            Self::try_from_png(std::fs::File::open(path)?)
        }

        pub fn try_from_png(reader: impl Read) -> io::Result<Self> {
            Self::try_from(png::Decoder::new(reader))
        }
    }

    impl<T: Read> TryFrom<Decoder<T>> for Image {
        type Error = std::io::Error;

        fn try_from(mut decoder: Decoder<T>) -> Result<Self, Self::Error> {
            decoder.set_transformations(Transformations::STRIP_16 | Transformations::EXPAND);
            let mut reader = decoder.read_info()?;
            let mut buffer = vec![0; reader.output_buffer_size()];
            let output_info = reader.next_frame(&mut buffer)?;
            if output_info.bit_depth != BitDepth::Eight { return Err(io::Error::new(InvalidData, format!("png::{:?} not yet supported by maulingmonkey-console-art", output_info.bit_depth))); }
            let width  = usize::try_from(output_info.width  ).map_err(|_| io::Error::new(InvalidData, "png width too large for maulingmonkey-console-art"))?;
            let height = usize::try_from(output_info.height ).map_err(|_| io::Error::new(InvalidData, "png height too large for maulingmonkey-console-art"))?;
            let mut src = buffer[..output_info.buffer_size()].iter().copied();

            let mut data = Vec::<RGB>::new();
            data.reserve(width * height);

            let src_eof = || { io::Error::new(InvalidData, "unexpected EOF decoding png") };
            let pal_req = || { io::Error::new(InvalidData, "missing palette for indexed color when decoding png") };
            let pal_oob = || { io::Error::new(InvalidData, "indexed color exceeds palette when decoding png") };

            let bytes_per_pixel = match output_info.color_type {
                ColorType::Grayscale        => 1,
                ColorType::GrayscaleAlpha   => 2,
                ColorType::Indexed          => 1,
                ColorType::Rgb              => 3,
                ColorType::Rgba             => 4,
            };

            let pad_scanline = output_info.line_size - width * bytes_per_pixel;

            for _y in 0..height {
                for _x in 0..width {
                    match output_info.color_type {
                        ColorType::Grayscale        => {
                            let v = src.next().ok_or_else(src_eof)?;
                            data.push(RGB(v,v,v));
                        },
                        ColorType::GrayscaleAlpha   => {
                            let v = src.next().ok_or_else(src_eof)? as usize;
                            let a = src.next().ok_or_else(src_eof)? as usize;
                            let v = (v * a / 255) as u8;
                            data.push(RGB(v,v,v));
                        },
                        ColorType::Indexed          => {
                            let pal  = reader.info().palette.as_ref().ok_or_else(pal_req)?;
                            let trns = reader.info().trns.as_ref().map(|p| &**p).unwrap_or(&[][..]);
                            let i = src.next().ok_or_else(src_eof)? as usize;
                            let r = *pal.get(3*i + 0).ok_or_else(pal_oob)? as usize;
                            let g = *pal.get(3*i + 1).ok_or_else(pal_oob)? as usize;
                            let b = *pal.get(3*i + 2).ok_or_else(pal_oob)? as usize;
                            let a = trns.get(i).copied().unwrap_or(0xFF) as usize;
                            let r = (r * a / 255) as u8;
                            let g = (g * a / 255) as u8;
                            let b = (b * a / 255) as u8;
                            data.push(RGB(r,g,b));
                        },
                        ColorType::Rgb              => {
                            let r = src.next().ok_or_else(src_eof)?;
                            let g = src.next().ok_or_else(src_eof)?;
                            let b = src.next().ok_or_else(src_eof)?;
                            data.push(RGB(r,g,b));
                        },
                        ColorType::Rgba             => {
                            let r = src.next().ok_or_else(src_eof)? as usize;
                            let g = src.next().ok_or_else(src_eof)? as usize;
                            let b = src.next().ok_or_else(src_eof)? as usize;
                            let a = src.next().ok_or_else(src_eof)? as usize;
                            let r = (r * a / 255) as u8;
                            let g = (g * a / 255) as u8;
                            let b = (b * a / 255) as u8;
                            data.push(RGB(r,g,b));
                        },
                    }
                }
                for _ in 0 .. pad_scanline { let _ = src.next(); }
            }
            Ok(Image { data, width, height })
        }
    }
}
