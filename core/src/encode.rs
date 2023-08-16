use std::fmt::{
    self,
    Formatter,
};

use crate::satisfy::Satisfier;

// =============================================================================
// Encode
// =============================================================================

// Traits

pub trait EncodeExt {
    fn write_str_encoded(&mut self, raw: &str, matcher: &impl Satisfier) -> fmt::Result;
}

// =============================================================================
// Implementation
// =============================================================================

impl EncodeExt for Formatter<'_> {
    fn write_str_encoded(&mut self, raw: &str, satisifer: &impl Satisfier) -> fmt::Result {
        let mut position = 0;

        loop {
            let rest = &raw[position..];

            if rest.is_empty() {
                break;
            }

            match satisifer.satisfies(rest) {
                0 => {
                    if let Some(c) = rest.chars().next() {
                        for b in c.encode_utf8(&mut [0; 4]).bytes() {
                            self.write_fmt(format_args!("%{:02X}", b))?;

                            position += 1;
                        }
                    }
                }
                n => {
                    self.write_str(&rest[..n])?;

                    position += n;
                }
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------

// Predicates

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
pub const fn is_reserved_ascii(b: u8) -> bool {
    match b {
        _ if is_general_delimiter_ascii(b) => true,
        _ if is_sub_delimiter_ascii(b) => true,
        _ => false,
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
pub const fn is_unreserved_ascii(b: u8) -> bool {
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
pub const fn is_general_delimiter_ascii(b: u8) -> bool {
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
pub const fn is_sub_delimiter_ascii(b: u8) -> bool {
    match b {
        | b'\x21'           // !
        | b'\x24'           // $
        | b'\x26'..=b'\x2c' // &, ', (, ), *, +, ,
        | b'\x3b'           // ;
        | b'\x3d' => true,  // =
        _ => false,
    }
}
