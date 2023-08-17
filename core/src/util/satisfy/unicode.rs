use crate::util::satisfy::Satisfy;

// =============================================================================
// Unicode
// =============================================================================

// Types

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
            .map(|p| (p..).find(|p| input.is_char_boundary(*p)).unwrap())
            .unwrap_or_else(|| input.len())
    }
}
