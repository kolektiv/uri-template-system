use std::sync::OnceLock;

// =============================================================================
// Encode
// =============================================================================

// Types

pub struct Encoding {
    pub allow_encoded: bool,
    pub allow: Box<dyn Fn(char) -> bool + Send + Sync>,
}

// -----------------------------------------------------------------------------

// Encoding

pub fn encode(input: &str, output: &mut String, encoding: &Encoding) {
    encode_str(input, output, encoding);
}

fn encode_str(input: &str, output: &mut String, encoding: &Encoding) {
    let mut state = State::Empty;

    for input in input.chars() {
        if encoding.allow_encoded {
            if let Some(input) = buffer(&mut state, output, encoding, input) {
                encode_char(input, output, encoding);
            }
        } else {
            encode_char(input, output, encoding);
        }
    }

    flush(&state, output, encoding);
}

fn encode_char(input: char, output: &mut String, encoding: &Encoding) {
    if (encoding.allow)(input) {
        encode_char_utf8(input, output);
    } else {
        encode_char_percent(input, output);
    }
}

fn encode_char_utf8(input: char, output: &mut String) {
    output.push(input);
}

fn encode_char_percent(input: char, output: &mut String) {
    input
        .encode_utf8(&mut [0; 4])
        .as_bytes()
        .iter()
        .for_each(|byte| {
            encode_byte_percent(*byte)
                .iter()
                .for_each(|char| output.push(*char))
        });
}

fn encode_byte_percent(input: u8) -> &'static [char; 3] {
    PERCENT_ENCODED
        .get_or_init(percent_encoded)
        .get(usize::from(input))
        .unwrap()
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

// State

enum State {
    Empty,
    Percent,
    HexDigit(char),
}

fn buffer(
    state: &mut State,
    output: &mut String,
    encoding: &Encoding,
    input: char,
) -> Option<char> {
    match &state {
        State::HexDigit(_) if is_hex_digit(input) => {
            complete(state, output, input);
            *state = State::Empty;
            None
        }
        State::Percent if is_hex_digit(input) => {
            *state = State::HexDigit(input);
            None
        }
        _ if is_percent(input) => {
            flush(state, output, encoding);

            *state = State::Percent;
            None
        }
        _ => {
            flush(state, output, encoding);

            *state = State::Empty;
            Some(input)
        }
    }
}

fn complete(state: &State, output: &mut String, input: char) {
    if let State::HexDigit(hex_digit) = state {
        encode_char_utf8('%', output);
        encode_char_utf8(*hex_digit, output);
        encode_char_utf8(input, output);
    }
}

fn flush(state: &State, output: &mut String, encoding: &Encoding) {
    match state {
        State::HexDigit(hex_digit) => {
            encode_char('%', output, encoding);
            encode_char(*hex_digit, output, encoding);
        }
        State::Percent => {
            encode_char('%', output, encoding);
        }
        _ => {}
    }
}

// -----------------------------------------------------------------------------

// Percent Encoding

static PERCENT_ENCODED: OnceLock<[[char; 3]; 256]> = OnceLock::new();

fn percent_encoded() -> [[char; 3]; 256] {
    [Vec::from_iter('0'..='9'), Vec::from_iter('A'..='F')]
        .concat()
        .iter()
        .flat_map(|a| {
            [Vec::from_iter('0'..='9'), Vec::from_iter('A'..='F')]
                .concat()
                .iter()
                .map(|b| ['%', *a, *b])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
