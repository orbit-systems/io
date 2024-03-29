#![warn(clippy::pedantic)]
mod const_hex_parse;
mod image;
mod name;

use std::{fs::File, io::Write};

use image::FontImage;
use name::name;
const FILES: &[FontImage] = &files![
    "0000",
    "0370",
    "2000",
    "2500",
    "2800",
    "FB00",
    "FF60",
];
const OUT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/target/io.abf");
const NAME: [u8; 16] = name("Io");

fn main() {
    let mut file = File::create(OUT).unwrap();
    file.write_all(&NAME).unwrap();
    let mut glyphs = FILES.iter().flat_map(|i| i.to_glyphs::<Vec<_>>()).collect::<Vec<_>>();
    glyphs.sort_by_key(|g| g.char);
    glyphs.dedup_by_key(|g| g.char as u32);
    for glyph in glyphs {
        file.write_all(&glyph.to_bytes()).unwrap();
    }
}
