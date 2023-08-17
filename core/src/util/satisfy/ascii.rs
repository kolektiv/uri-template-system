use crate::util::satisfy::Satisfy;

// =============================================================================
// ASCII
// =============================================================================

// Types

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
            .unwrap_or_else(|| input.len())
    }
}
