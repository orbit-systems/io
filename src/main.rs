mod const_hex_parse;
mod image;
mod name;

use std::{fs::File, io::Write};

use image::FontImage;
use name::name;
// IMPORTANT!
// order the numbers/files correctly,
// and make sure there's no overlap between the codepoints
const FILES: &[FontImage] = &files!["00"];
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
