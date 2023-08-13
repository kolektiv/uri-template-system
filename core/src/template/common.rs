use std::sync::OnceLock;

use crate::codec::Encoding;

// =============================================================================
// Common
// =============================================================================

// Expansion

static RESERVED: OnceLock<Encoding> = OnceLock::new();
static UNRESERVED: OnceLock<Encoding> = OnceLock::new();

pub fn reserved() -> &'static Encoding {
    RESERVED.get_or_init(|| Encoding {
        allow_encoded: true,
        allow: Box::new(|c| is_unreserved(c) || is_reserved(c)),
    })
}

pub fn unreserved() -> &'static Encoding {
    UNRESERVED.get_or_init(|| Encoding {
        allow_encoded: false,
        allow: Box::new(is_unreserved),
    })
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_reserved(c: char) -> bool {
    match c {
        _ if is_gen_delim(c) => true,
        _ if is_sub_delim(c) => true,
        _ => false,
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_unreserved(c: char) -> bool {
    match c {
        | '\x30'..='\x39'
        | '\x41'..='\x5a'
        | '\x61'..='\x7a'
        | '\x2d'..='\x2e'
        | '\x5f'
        | '\x7e' => true,
        _ => false,
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_gen_delim(c: char) -> bool {
    match c {
        | '\x23'
        | '\x2f'
        | '\x3a'
        | '\x3f'
        | '\x40'
        | '\x5b'
        | '\x5d' => true,
        _ => false,
    }
}

#[rustfmt::skip]
#[allow(clippy::match_like_matches_macro)]
#[inline]
const fn is_sub_delim(c: char) -> bool {
    match c {
        | '\x21'
        | '\x24'
        | '\x26'..='\x2c'
        | '\x3b'
        | '\x3d' => true,
        _ => false,
    }
}
