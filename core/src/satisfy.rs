// =============================================================================
// Satisfy
// =============================================================================

// Traits

pub trait Satisfier {
    fn satisfies(&self, input: &str) -> usize;
}

impl Satisfier for Box<dyn Satisfier> {
    fn satisfies(&self, input: &str) -> usize {
        self.as_ref().satisfies(input)
    }
}

// =============================================================================
// Implementation
// =============================================================================

// Composites

impl<S1, S2> Satisfier for (S1, S2)
where
    S1: Satisfier,
    S2: Satisfier,
{
    fn satisfies(&self, input: &str) -> usize {
        let mut pos = 0;
        let mut exhausted = (true, true);

        loop {
            if input[pos..].is_empty() {
                break;
            }

            if exhausted.0 {
                match self.0.satisfies(&input[pos..]) {
                    n if n > 0 => {
                        pos += n;
                        exhausted.1 = true;
                    }
                    _ => {}
                }

                exhausted.0 = false;
            }

            if exhausted.1 {
                match self.1.satisfies(&input[pos..]) {
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

impl<S1, S2, S3> Satisfier for (S1, S2, S3)
where
    S1: Satisfier,
    S2: Satisfier,
    S3: Satisfier,
{
    fn satisfies(&self, input: &str) -> usize {
        let mut pos = 0;
        let mut exhausted = (true, true, true);

        loop {
            if input[pos..].is_empty() {
                break;
            }

            if exhausted.1 {
                match self.0.satisfies(&input[pos..]) {
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
                match self.1.satisfies(&input[pos..]) {
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
                match self.2.satisfies(&input[pos..]) {
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

impl<P> Satisfier for Ascii<P>
where
    P: Fn(u8) -> bool,
{
    fn satisfies(&self, input: &str) -> usize {
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

impl<P> Satisfier for Unicode<P>
where
    P: Fn(char) -> bool,
{
    fn satisfies(&self, input: &str) -> usize {
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

impl Satisfier for PercentEncoded {
    fn satisfies(&self, input: &str) -> usize {
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
