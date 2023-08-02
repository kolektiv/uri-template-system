use std::sync::OnceLock;

// =============================================================================
// Encode
// =============================================================================

// Types

// #[derive(Debug)]
pub struct Encoding {
    pub allow_encoded: bool,
    pub allow: Box<dyn Fn(char) -> bool + Send + Sync>,
}

enum Buffer {
    Empty,
    Percent,
    HexDigit(char),
}

impl Buffer {
    fn extend(&mut self, output: &mut String, encoding: &Encoding, input: char) -> Option<char> {
        match &self {
            Self::HexDigit(_) if is_hex_digit(input) => {
                self.complete(output, input);

                *self = Self::Empty;
                None
            }
            Self::Percent if is_hex_digit(input) => {
                *self = Self::HexDigit(input);
                None
            }
            _ if is_percent(input) => {
                self.flush(output, encoding);

                *self = Self::Percent;
                None
            }
            _ => {
                self.flush(output, encoding);

                *self = Self::Empty;
                Some(input)
            }
        }
    }

    fn complete(&self, output: &mut String, input: char) {
        match self {
            Buffer::HexDigit(hex_digit) => {
                push_char_utf8('%', output);
                push_char_utf8(*hex_digit, output);
                push_char_utf8(input, output);
            }
            _ => {}
        }
    }

    fn flush(&self, output: &mut String, encoding: &Encoding) {
        match self {
            Buffer::HexDigit(hex_digit) => {
                push_char('%', output, encoding);
                push_char(*hex_digit, output, encoding);
            }
            Buffer::Percent => {
                push_char('%', output, encoding);
            }
            _ => {}
        }
    }
}

// -----------------------------------------------------------------------------

// Functions

pub fn encode(input: &str, output: &mut String, encoding: &Encoding) {
    println!("encoding: '{input}'");
    push_str(input, output, encoding);
}

fn push_str(input: &str, output: &mut String, encoding: &Encoding) {
    let mut buffer = Buffer::Empty;

    for input in input.chars() {
        if encoding.allow_encoded {
            if let Some(input) = buffer.extend(output, encoding, input) {
                push_char(input, output, encoding);
            }
        } else {
            push_char(input, output, encoding);
        }
    }

    buffer.flush(output, encoding);
}

fn push_char(input: char, output: &mut String, encoding: &Encoding) {
    if (&encoding.allow)(input) {
        push_char_utf8(input, output);
    } else {
        push_char_percent(input, output);
    }
}

fn push_char_utf8(input: char, output: &mut String) {
    output.push(input);
}

fn push_char_percent(input: char, output: &mut String) {
    input
        .encode_utf8(&mut [0; 4])
        .as_bytes()
        .iter()
        .for_each(|byte| {
            get_encoded(*byte)
                .iter()
                .for_each(|char| output.push(*char))
        });
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_hex_digit(c: char) -> bool {
    match c {
        _ if c.is_ascii_hexdigit() => true,
        _ => false,
    }
}

#[allow(clippy::match_like_matches_macro)]
#[rustfmt::skip]
fn is_percent(c: char) -> bool {
    match c {
        _ if c == '%' => true,
        _ => false,
    }
}

// -----------------------------------------------------------------------------

// Lookups

static ENCODED: OnceLock<[[char; 3]; 256]> = OnceLock::new();

fn init_encoded() -> [[char; 3]; 256] {
    [Vec::from_iter('0'..='9'), Vec::from_iter('A'..='F')]
        .concat()
        .iter()
        .map(|a| {
            [Vec::from_iter('0'..='9'), Vec::from_iter('A'..='F')]
                .concat()
                .iter()
                .map(|b| ['%', *a, *b])
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn get_encoded(input: u8) -> &'static [char; 3] {
    ENCODED
        .get_or_init(init_encoded)
        .get(usize::from(input))
        .unwrap()
}
