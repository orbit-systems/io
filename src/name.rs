use std::cmp::min;

const NAME_LIMIT: usize = 14;

macro_rules! g {
    ($arr: expr, $i: literal) => {{
        if $i < $arr.len() {
            $arr[$i]
        } else {
            0
        }
    }};
}

pub(crate) const fn name(s: &str) -> [u8; 16] {
    assert!(s.is_ascii());
    let s = s.as_bytes();
    [
        0,
        g!(s, 0),
        g!(s, 1),
        g!(s, 2),
        g!(s, 3),
        g!(s, 4),
        g!(s, 5),
        g!(s, 6),
        g!(s, 7),
        g!(s, 8),
        g!(s, 9),
        g!(s, 10),
        g!(s, 11),
        g!(s, 12),
        g!(s, 13),
        0,
    ]
}
