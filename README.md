6x12 pixel font. Go to [/src](/src) to see source image. 

# Compiling

The image files get included in the compiled binary. Simply running the code should output the file at `/target/io.abf`.

Files should be named `src_U+X` where X is the starting codepoint of the glyph. The glyph data is assumed to be contiguious.
Any glyph with a color other than black / white is assumed to be a skipped glyph, and is conventionally represented by filling the rectangle with red.

To add a file, first add the file itself, then modify the source code at [/src/main.rs](/src/main.rs):
```rs
const FILES: &[FontImage] = &files!["00"];
```
to contain the *exact* string of the codepoint part of the file name.

For example, if you've added `src_U+030.png` and `src_U+40.png`, you would modify it to be like this:
```rs
const FILES: &[FontImage] = &files!["00", "030", "40"];
```
and not this:
```rs
const FILES: &[FontImage] = &files!["00", "30", "40"];
```
or this:
```rs
const FILES: &[FontImage] = &files!["00", "U+0030", "U+0040"];
```
Since the string is used for both finding the codepoint AND the source image.

In general, stick to 2, 4, and 6 letters long representation of codepoints.

# Binary format

The binary format is divided into 16 byte chunks. A chunk represents a glyph except for the first chunk which contains the name of the font.

## Name format

|    0    |             1..14             |    15   |
|---------|-------------------------------|---------|
| (empty) | name (ascii, null terminated) | (empty) |

## Glyph format

|    0..3   |   4..  |
|-----------|--------|
| codepoint |  rows  |

where a byte represents a row.
