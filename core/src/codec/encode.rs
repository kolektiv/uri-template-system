use std::sync::OnceLock;

// =============================================================================
// Encode
// =============================================================================

// Types

#[derive(Debug)]
pub struct Encoding<P>
where
    P: Fn(char) -> bool,
{
    pub allow_encoded: bool,
    pub allow: P,
}

enum Buffer {
    Empty,
    Percent,
    HexDigit(char),
}

// -----------------------------------------------------------------------------

// Functions

pub fn encode<P>(input: &str, output: &mut String, encoding: &Encoding<P>)
where
    P: Fn(char) -> bool,
{
    push_str(input, output, encoding);
}

fn push_str<P>(input: &str, output: &mut String, encoding: &Encoding<P>)
where
    P: Fn(char) -> bool,
{
    let mut buffer = Buffer::Empty;

    for input in input.chars() {
        if encoding.allow_encoded {
            match buffer {
                Buffer::HexDigit(hex_digit) => {
                    if is_hex_digit(input) {
                        push_char_utf8('&', output);
                        push_char_utf8(hex_digit, output);
                        push_char_utf8(input, output);

                        buffer = Buffer::Empty;

                        continue;
                    } else {
                        push_char('%', output, encoding);
                        push_char(hex_digit, output, encoding);
                    }
                }
                Buffer::Percent => {
                    if is_hex_digit(input) {
                        buffer = Buffer::HexDigit(input);

                        continue;
                    } else {
                        push_char('%', output, encoding);
                    }
                }
                _ => {}
            }

            if is_percent(input) {
                buffer = Buffer::Percent;

                continue;
            }

            buffer = Buffer::Empty;
        }

        push_char(input, output, encoding);
    }
}

fn push_char<P>(input: char, output: &mut String, encoding: &Encoding<P>)
where
    P: Fn(char) -> bool,
{
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

// -----------------------------------------------------------------------------

// Lookups

static ENCODED: OnceLock<[[char; 3]; 128]> = OnceLock::new();

fn init_encoded() -> [[char; 3]; 128] {
    Vec::from_iter('0'..='7')
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

// -----------------------------------------------------------------------------

// Predicates

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
