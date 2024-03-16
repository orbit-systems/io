#[derive(Debug)]
enum HexDgt {
    X0 = 0x0,
    X1 = 0x1,
    X2 = 0x2,
    X3 = 0x3,
    X4 = 0x4,
    X5 = 0x5,
    X6 = 0x6,
    X7 = 0x7,
    X8 = 0x8,
    X9 = 0x9,
    XA = 0xA,
    XB = 0xB,
    XC = 0xC,
    XD = 0xD,
    XE = 0xE,
    XF = 0xF,
}
impl HexDgt {
    const fn from_bt(c: u8) -> Option<Self> {
        match c {
            b'0' => Some(Self::X0),
            b'1' => Some(Self::X1),
            b'2' => Some(Self::X2),
            b'3' => Some(Self::X3),
            b'4' => Some(Self::X4),
            b'5' => Some(Self::X5),
            b'6' => Some(Self::X6),
            b'7' => Some(Self::X7),
            b'8' => Some(Self::X8),
            b'9' => Some(Self::X9),
            b'A' | b'a' => Some(Self::XA),
            b'B' | b'b' => Some(Self::XB),
            b'C' | b'c' => Some(Self::XC),
            b'D' | b'd' => Some(Self::XD),
            b'E' | b'e' => Some(Self::XE),
            b'F' | b'f' => Some(Self::XF),
            _ => None,
        }
    }
    const fn bt_to_u32(c: u8) -> Option<u32> {
        if let Some(v) = Self::from_bt(c) {
            Some(v as u32)
        } else {
            None
        }
    }
}
macro_rules! q {
    ($v: expr) => {{
        if let Some(v) = $v {
            v
        } else {
            return None;
        }
    }};
}
macro_rules! num {
    ($v: expr) => {
        q!(HexDgt::bt_to_u32($v))
    };
    ($v: expr, $s: literal) => {
        (q!(HexDgt::bt_to_u32($v)) << ($s * 4u32))
    };
}
#[inline]
const fn bstr_to_u32(v: &[u8]) -> Option<u32> {
    match *v {
        [d0] => Some(num!(d0)),
        [d1, d0] => Some(num!(d1, 1) | num!(d0)),
        [d2, d1, d0] => Some(num!(d2, 2) | num!(d1, 1) | num!(d0)),
        [d3, d2, d1, d0] => Some(num!(d3, 3) | num!(d2, 2) | num!(d1, 1) | num!(d0)),
        [d4, d3, d2, d1, d0] => {
            Some(num!(d4, 4) | num!(d3, 3) | num!(d2, 2) | num!(d1, 1) | num!(d0))
        }
        [d5, d4, d3, d2, d1, d0] => {
            Some(num!(d5, 5) | num!(d4, 4) | num!(d3, 3) | num!(d2, 2) | num!(d1, 1) | num!(d0))
        }
        _ => None,
    }
}
#[inline]
const fn str_to_u32(v: &str) -> Option<u32> {
    bstr_to_u32(v.as_bytes())
}
#[inline]
const fn bstr_to_char(v: &[u8]) -> Option<char> {
    let v = q!(bstr_to_u32(v));
    char::from_u32(v)
}
#[inline]
const fn parse_codepoint(v: &str) -> Option<char> {
    let v = v.as_bytes();
    let v = match v {
        &[b'U', b'+', ..] | &[b'u', b'+', ..] => v.split_at(2).1,
        _ => v,
    };
    bstr_to_char(v)
}
#[inline]
pub(crate) const fn parse_codepoint_p(v: &str) -> char {
    if let Some(v) = parse_codepoint(v) {
        v
    } else {
        panic!()
    }
}
