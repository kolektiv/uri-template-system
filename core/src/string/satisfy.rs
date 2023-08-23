pub mod tuple_2;
pub mod tuple_3;

use crate::string::Satisfy;

// =============================================================================
// Satisfy
// =============================================================================

// Common

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

// -----------------------------------------------------------------------------

// Implementations

impl Satisfy for Box<dyn Satisfy> {
    fn satisfy(&self, input: &str) -> usize {
        self.as_ref().satisfy(input)
    }
}

pub struct Ascii<P>
where
    P: Fn(u8) -> bool,
{
    predicate: P,
}

impl<P> Ascii<P>
where
    P: Fn(u8) -> bool + 'static,
{
    pub const fn new(predicate: P) -> Self {
        Self { predicate }
    }
}

impl<P> Satisfy for Ascii<P>
where
    P: Fn(u8) -> bool,
{
    fn satisfy(&self, input: &str) -> usize {
        input
            .bytes()
            .position(|b| !b.is_ascii() || !(self.predicate)(b))
            .unwrap_or(input.len())
    }
}

pub struct PercentEncoded;

impl Satisfy for PercentEncoded {
    fn satisfy(&self, input: &str) -> usize {
        let mut pos = 0;

        loop {
            match input[pos..].as_bytes() {
                [b'%', a, b, ..] if a.is_ascii_hexdigit() && b.is_ascii_hexdigit() => pos += 3,
                _ => break,
            }
        }

        pos
    }
}

pub struct Unicode<P>
where
    P: Fn(char) -> bool,
{
    predicate: P,
}

impl<P> Unicode<P>
where
    P: Fn(char) -> bool + 'static,
{
    pub const fn new(predicate: P) -> Self {
        Self { predicate }
    }
}

impl<P> Satisfy for Unicode<P>
where
    P: Fn(char) -> bool,
{
    fn satisfy(&self, input: &str) -> usize {
        input
            .chars()
            .position(|c| c.is_ascii() || !(self.predicate)(c))
            .map_or_else(
                || input.len(),
                |p| (p..p + 4).find(|p| input.is_char_boundary(*p)).unwrap(),
            )
    }
}
