pub mod ascii;
pub mod percent_encoded;
pub mod tuple;
pub mod unicode;

use crate::util::satisfy::{
    ascii::Ascii,
    percent_encoded::PercentEncoded,
};

// =============================================================================
// Satisfy
// =============================================================================

// Traits

pub trait Satisfy {
    fn satisfy(&self, input: &str) -> usize;
}

impl Satisfy for Box<dyn Satisfy> {
    fn satisfy(&self, input: &str) -> usize {
        self.as_ref().satisfy(input)
    }
}

// -----------------------------------------------------------------------------

// Standards

pub fn unreserved() -> impl Satisfy {
    Ascii::new(is_unreserved_ascii)
}

pub fn unreserved_or_reserved() -> impl Satisfy {
    (
        Ascii::new(|b| is_unreserved_ascii(b) || is_reserved_ascii(b)),
        PercentEncoded,
    )
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_unreserved_ascii(b: u8) -> bool {
    match b {
        | b'\x61'..=b'\x7a' // a..z
        | b'\x41'..=b'\x5a' // A..Z
        | b'\x30'..=b'\x39' // 0..9
        | b'\x2d'..=b'\x2e' // -, .
        | b'\x5f'           // _
        | b'\x7e' => true,  // ~
        _ => false,
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_reserved_ascii(b: u8) -> bool {
    match b {
        _ if is_general_delimiter_ascii(b) => true,
        _ if is_sub_delimiter_ascii(b) => true,
        _ => false,
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_general_delimiter_ascii(b: u8) -> bool {
    match b {
        | b'\x23'           // #
        | b'\x2f'           // /
        | b'\x3a'           // :
        | b'\x3f'           // ?
        | b'\x40'           // @
        | b'\x5b'           // [
        | b'\x5d' => true,  // ]
        _ => false,
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_sub_delimiter_ascii(b: u8) -> bool {
    match b {
        | b'\x21'           // !
        | b'\x24'           // $
        | b'\x26'..=b'\x2c' // &, ', (, ), *, +, ,
        | b'\x3b'           // ;
        | b'\x3d' => true,  // =
        _ => false,
    }
}
