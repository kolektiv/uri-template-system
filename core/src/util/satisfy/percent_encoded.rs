use crate::util::satisfy::Satisfy;

// =============================================================================
// Percent Encoded
// =============================================================================

// Types

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
