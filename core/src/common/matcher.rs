// =============================================================================
// Matcher
// =============================================================================

// Traits

pub trait Matcher {
    fn matches(&self, input: &str) -> usize;
}

// =============================================================================
// Matchers
// =============================================================================

// Composites

impl<M1, M2> Matcher for (M1, M2)
where
    M1: Matcher,
    M2: Matcher,
{
    fn matches(&self, input: &str) -> usize {
        let mut pos = 0;
        let mut exhausted = (true, true);

        loop {
            if input[pos..].is_empty() {
                break;
            }

            if exhausted.0 {
                match self.0.matches(&input[pos..]) {
                    n if n > 0 => {
                        pos += n;
                        exhausted.1 = true;
                    }
                    _ => {}
                }

                exhausted.0 = false;
            }

            if exhausted.1 {
                match self.1.matches(&input[pos..]) {
                    n if n > 0 => {
                        pos += n;
                        exhausted.0 = true;
                    }
                    _ => {}
                }

                exhausted.1 = false;
            }

            if !(exhausted.0 || exhausted.1) {
                break;
            }
        }

        pos
    }
}

impl<M1, M2, M3> Matcher for (M1, M2, M3)
where
    M1: Matcher,
    M2: Matcher,
    M3: Matcher,
{
    fn matches(&self, input: &str) -> usize {
        let mut pos = 0;
        let mut exhausted = (true, true, true);

        loop {
            if input[pos..].is_empty() {
                break;
            }

            if exhausted.1 {
                match self.0.matches(&input[pos..]) {
                    n if n > 0 => {
                        pos += n;
                        exhausted.1 = true;
                        exhausted.2 = true;
                    }
                    _ => {}
                }

                exhausted.0 = false;
            }

            if exhausted.1 {
                match self.1.matches(&input[pos..]) {
                    n if n > 0 => {
                        pos += n;
                        exhausted.0 = true;
                        exhausted.2 = true;
                    }
                    _ => {}
                }

                exhausted.1 = false;
            }

            if exhausted.2 {
                match self.2.matches(&input[pos..]) {
                    n if n > 0 => {
                        pos += n;
                        exhausted.0 = true;
                        exhausted.1 = true;
                    }
                    _ => {}
                }

                exhausted.2 = false;
            }

            if !(exhausted.0 || exhausted.1 || exhausted.2) {
                break;
            }
        }

        pos
    }
}

// -----------------------------------------------------------------------------

// ASCII

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

impl<P> Matcher for Ascii<P>
where
    P: Fn(u8) -> bool,
{
    fn matches(&self, input: &str) -> usize {
        input
            .bytes()
            .position(|b| !b.is_ascii() || !(self.predicate)(b))
            .unwrap_or_else(|| input.len())
    }
}

// -----------------------------------------------------------------------------

// Unicode

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

impl<P> Matcher for Unicode<P>
where
    P: Fn(char) -> bool,
{
    fn matches(&self, input: &str) -> usize {
        input
            .chars()
            .position(|c| c.is_ascii() || !(self.predicate)(c))
            .map(|p| (p..).find(|p| input.is_char_boundary(*p)).unwrap())
            .unwrap_or_else(|| input.len())
    }
}

// -----------------------------------------------------------------------------

// Percent Encoded

pub struct PercentEncoded;

impl Matcher for PercentEncoded {
    fn matches(&self, input: &str) -> usize {
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
