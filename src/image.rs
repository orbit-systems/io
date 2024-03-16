use std::char;

use image::{GenericImageView, ImageFormat, Rgba};

#[macro_export]
macro_rules! file {
    ($str: literal) => {{
        const CHAR: char = const_hex_parse::parse_codepoint_p($str);
        const IMAGE: &[u8] = include_bytes!(concat!("src_U+", $str, ".png"));
        FontImage {
            start: CHAR,
            image: IMAGE,
        }
    }};
}
#[macro_export]
macro_rules! files {
    ($($str: literal),* $(,)*) => {
        [$(file!($str)),*]
    };
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct FontImage {
    pub(crate) start: char,
    pub(crate) image: &'static [u8],
}
impl FontImage {
    pub(crate) fn to_glyphs<I: FromIterator<Glyph>>(self) -> I {
        const WIDTH: u32 = 6;
        const HEIGHT: u32 = 12;
        let image = image::load_from_memory_with_format(self.image, ImageFormat::Png).unwrap();
        let (width, height) = (image.width(), image.height());
        let (rel_width, rel_height) = (width / WIDTH, height / HEIGHT);
        (0..(rel_width * rel_height))
            .filter_map(|i| {
                const TOLERANCE: u8 = 4;
                const INV_TOLERANCE: u8 = 255 - TOLERANCE;
                let char = char::from_u32(self.start as u32 + i)?;
                let (rel_x, rel_y) = (i % rel_width, i / rel_width);
                let (start_x, start_y) = (rel_x * WIDTH, rel_y * HEIGHT);
                let mut data = [0u8; 12];
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        let (abs_x, abs_y) = (x + start_x, y + start_y);
                        match image.get_pixel(abs_x, abs_y) {
                            Rgba([0..=TOLERANCE, 0..=TOLERANCE, 0..=TOLERANCE, _]) => {}
                            Rgba(
                                [INV_TOLERANCE..=255, INV_TOLERANCE..=255, INV_TOLERANCE..=255, _],
                            ) => {
                                data[y as usize] |= 1 << x;
                            }
                            _ => return None,
                        };
                    }
                }
                Some(Glyph { char, data })
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Glyph {
    pub(crate) char: char,
    pub(crate) data: [u8; 12],
}
impl Glyph {
    pub(crate) const fn to_bytes(self) -> [u8; 16] {
        if cfg!(target_endian = "big") {
            let [b0, b1, b2, b3] = (self.char as u32).to_le_bytes();
            [
                b0,
                b1,
                b2,
                b3,
                self.data[0],
                self.data[1],
                self.data[2],
                self.data[3],
                self.data[4],
                self.data[5],
                self.data[6],
                self.data[7],
                self.data[8],
                self.data[9],
                self.data[10],
                self.data[11],
            ]
        } else {
            unsafe { std::mem::transmute(self) }
        }
    }
}
