use std::fmt::{
    self,
    Write,
};

use crate::util::satisfy::Satisfy;

// =============================================================================
// Encode
// =============================================================================

// Traits

pub trait Encode {
    fn encode(&mut self, raw: &str, matcher: &impl Satisfy) -> fmt::Result;
}

impl<T> Encode for T
where
    T: Write,
{
    fn encode(&mut self, raw: &str, satisifer: &impl Satisfy) -> fmt::Result {
        let mut position = 0;

        loop {
            let rest = &raw[position..];

            if rest.is_empty() {
                break;
            }

            match satisifer.satisfy(rest) {
                0 => {
                    if let Some(c) = rest.chars().next() {
                        for b in c.encode_utf8(&mut [0; 4]).bytes() {
                            self.write_fmt(format_args!("%{b:02X}"))?;

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
