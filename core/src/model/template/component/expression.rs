mod modifier;
mod operator;
mod variable;

use std::fmt::Write;

use crate::{
    model::{
        template::component::expression::{
            modifier::Modifier,
            operator::Operator,
            variable::VariableList,
        },
        value::{
            Value,
            Values,
        },
    },
    process::{
        expand::{
            Expand,
            ExpandError,
        },
        parse::{
            Parse,
            ParseError,
            TryParse,
        },
    },
    util::{
        encode::Encode,
        satisfy::{
            self,
            Satisfy,
        },
    },
};

// =============================================================================
// Expression
// =============================================================================

// Types

#[derive(Debug, Eq, PartialEq)]
pub struct Expression<'t> {
    operator: Option<Operator>,
    variable_list: VariableList<'t>,
}

impl<'t> Expression<'t> {
    const fn new(operator: Option<Operator>, variable_list: VariableList<'t>) -> Self {
        Self {
            operator,
            variable_list,
        }
    }
}

// -----------------------------------------------------------------------------

// Parse

impl<'t> TryParse<'t> for Expression<'t> {
    fn try_parse(raw: &'t str, global: usize) -> Result<(usize, Self), ParseError> {
        let mut parsed_operator = None;
        let mut parsed_variable_list = Vec::new();
        let mut state = ExpressionState::default();

        loop {
            let rest = &raw[state.position..];

            match &state.next {
                ExpressionNext::OpeningBrace if rest.starts_with('{') => {
                    state.next = ExpressionNext::Operator;
                    state.position += 1;
                }
                ExpressionNext::OpeningBrace => {
                    return Err(ParseError::UnexpectedInput {
                        position: global + state.position,
                        message: "unexpected input when parsing expression component".into(),
                        expected: "opening brace ('{')".into(),
                    });
                }
                ExpressionNext::Operator => {
                    let (position, operator) =
                        Option::<Operator>::parse(rest, global + state.position);

                    parsed_operator = operator;
                    state.next = ExpressionNext::VariableList;
                    state.position += position;
                }
                ExpressionNext::VariableList => {
                    match VariableList::try_parse(rest, global + state.position) {
                        Ok((position, variable_list)) => {
                            parsed_variable_list.extend(variable_list);
                            state.next = ExpressionNext::ClosingBrace;
                            state.position += position;
                        }
                        Err(err) => return Err(err),
                    }
                }
                ExpressionNext::ClosingBrace if rest.starts_with('}') => {
                    state.position += 1;

                    return Ok((
                        state.position,
                        Self::new(parsed_operator, parsed_variable_list),
                    ));
                }
                ExpressionNext::ClosingBrace => {
                    return Err(ParseError::UnexpectedInput {
                        position: global + state.position,
                        message: "unexpected input when parsing expression component".into(),
                        expected: "closing brace ('}')".into(),
                    });
                }
            }
        }
    }
}

#[derive(Default)]
struct ExpressionState {
    next: ExpressionNext,
    position: usize,
}

#[derive(Default)]
enum ExpressionNext {
    #[default]
    OpeningBrace,
    Operator,
    VariableList,
    ClosingBrace,
}

// -----------------------------------------------------------------------------

// Expand

impl<'t> Expand for Expression<'t> {
    #[allow(clippy::cognitive_complexity)] // TODO: Reduce?
    #[allow(clippy::equatable_if_let)]
    #[allow(clippy::too_many_lines)]
    fn expand(&self, values: &Values, write: &mut impl Write) -> Result<(), ExpandError> {
        let behaviour = self
            .operator
            .as_ref()
            .map_or(&operator::DEFAULT_BEHAVIOUR, Operator::behaviour);

        let matcher = behaviour.allow.matcher();
        let mut first = true;

        for (var_name, modifier) in &self.variable_list {
            // Lookup the value for the scanned variable name, and then
            //
            // * If the varname is unknown or corresponds to a variable with an undefined
            //   value (Section 2.3), then skip to the next varspec.

            let value = match values.get(var_name.name()) {
                Some(value) if value.defined() => value,
                _ => continue,
            };

            // * If this is the first defined variable for this expression, append the first
            //   string for this expression type to the result string and remember that it
            //   has been done.  Otherwise, append the sep string to the result string.

            if first {
                if let Some(c) = behaviour.first {
                    write.write_char(c)?;
                }

                first = false;
            } else {
                write.write_char(behaviour.sep)?;
            }

            if let Value::Item(value) = value {
                // If this variable's value is a string, then

                if behaviour.named {
                    // * if named is true, append the varname to the result string using the same
                    //   encoding process as for literals, and

                    write.encode(var_name.name(), &satisfy::unreserved_or_reserved())?;

                    if value.is_empty() {
                        // + if the value is empty, append the ifemp string to the result string and
                        //   skip to the next varspec;

                        if let Some(c) = behaviour.ifemp {
                            write.write_char(c)?;
                        }
                    } else {
                        // + otherwise, append "=" to the result string.

                        write.write_char('=')?;
                    }
                }

                match modifier {
                    Some(Modifier::Prefix(length)) => {
                        // * if a prefix modifier is present and the prefix length is less than the
                        //   value string length in number of Unicode characters, append that number
                        //   of characters from the beginning of the value string to the result
                        //   string, after pct-encoding any characters that are not in the allow
                        //   set, while taking care not to split multi-octet or pct-encoded triplet
                        //   characters that represent a single Unicode code point;

                        let pos: usize = value.chars().take(*length).map(char::len_utf8).sum();

                        write.encode(&value[..pos], &matcher)?;
                    }
                    _ => {
                        // * otherwise, append the value to the result string after pct-encoding any
                        //   characters that are not in the allow set.

                        write.encode(value, &matcher)?;
                    }
                };
            } else if let Some(Modifier::Explode) = modifier {
                // else if an explode modifier is given, then

                if behaviour.named {
                    // * if named is true, then for each defined list member or array (name, value)
                    //   pair with a defined value, do:

                    if let Value::AssociativeArray(value) = value {
                        let mut first = true;

                        for (name, value) in value {
                            // + if this is not the first defined member/value, append the sep
                            //   string to the result string;

                            if first {
                                first = false;
                            } else {
                                write.write_char(behaviour.sep)?;
                            }

                            // + if this is a pair, append the name to the result string using the
                            //   same encoding process as for literals;

                            write.encode(name, &satisfy::unreserved_or_reserved())?;

                            // + if the member/value is empty, append the ifemp string to the result
                            //   string; otherwise, append "=" and the member/value to the result
                            //   string, after pct-encoding any member/value characters that are not
                            //   in the allow set.

                            if value.is_empty() {
                                if let Some(c) = behaviour.ifemp {
                                    write.write_char(c)?;
                                }
                            } else {
                                write.write_char('=')?;
                                write.encode(value, &matcher)?;
                            }
                        }
                    } else if let Value::List(value) = value {
                        let mut first = true;

                        for value in value {
                            // + if this is not the first defined member/value, append the sep
                            //   string to the result string;

                            if first {
                                first = false;
                            } else {
                                write.write_char(behaviour.sep)?;
                            }

                            // + if this is a list, append the varname to the result string using
                            //   the same encoding process as for literals;

                            write.encode(var_name.name(), &satisfy::unreserved_or_reserved())?;

                            // + if the member/value is empty, append the ifemp string to the result
                            //   string; otherwise, append "=" and the member/value to the result
                            //   string, after pct-encoding any member/value characters that are not
                            //   in the allow set.

                            if value.is_empty() {
                                if let Some(c) = behaviour.ifemp {
                                    write.write_char(c)?;
                                }
                            } else {
                                write.write_char('=')?;
                                write.encode(value, &matcher)?;
                            }
                        }
                    }
                } else {
                    // * else if named is false, then

                    if let Value::AssociativeArray(value) = value {
                        // + if this is an array of (name, value) pairs, append each pair with a
                        //   defined value to the result string as "name=value", after pct-encoding
                        //   any characters that are not in the allow set, with the sep string
                        //   appended to the result between each defined pair.

                        let mut first = true;

                        for (name, value) in value {
                            if !value.is_empty() {
                                if first {
                                    first = false;
                                } else {
                                    write.write_char(behaviour.sep)?;
                                }
                            }

                            write.encode(name, &matcher)?;
                            write.write_char('=')?;
                            write.encode(value, &matcher)?;
                        }
                    } else if let Value::List(value) = value {
                        // + if this is a list, append each defined list member to the result
                        //   string, after pct-encoding any characters that are not in the allow
                        //   set, with the sep string appended to the result between each defined
                        //   list member.

                        let mut first = true;

                        for value in value {
                            if !value.is_empty() {
                                if first {
                                    first = false;
                                } else {
                                    write.write_char(behaviour.sep)?;
                                }
                            }

                            write.encode(value, &matcher)?;
                        }
                    }
                }
            } else {
                // else if no explode modifier is given, then

                if behaviour.named {
                    // * if named is true, append the varname to the result string using the same
                    //   encoding process as for literals, and

                    write.encode(var_name.name(), &satisfy::unreserved_or_reserved())?;

                    // + if the value is empty, append the ifemp string to the result string and
                    //   skip to the next varspec;
                    // + otherwise, append "=" to the result string; and

                    // NOTE: Empty values are not meaningful currently, so this logic is skipped for
                    // now

                    write.write_char('=')?;
                }

                if let Value::AssociativeArray(value) = value {
                    // * if this variable's value is an associative array or any other form of
                    //   paired (name, value) structure, append each pair with defined value to the
                    //   result string as "name,value", after pct-encoding any characters that are
                    //   not in the allow set, with a comma (",") appended to the result between
                    //   each defined pair.

                    let mut first = true;

                    for (name, value) in value {
                        if !value.is_empty() {
                            if first {
                                first = false;
                            } else {
                                write.write_char(',')?;
                            }

                            write.encode(name, &matcher)?;
                            write.write_char(',')?;
                            write.encode(value, &matcher)?;
                        }
                    }
                } else if let Value::List(value) = value {
                    // * if this variable's value is a list, append each defined list member to the
                    //   result string, after pct-encoding any characters that are not in the allow
                    //   set, with a comma (",") appended to the result between each defined list
                    //   member;

                    let mut first = true;

                    for value in value {
                        if !value.is_empty() {
                            if first {
                                first = false;
                            } else {
                                write.write_char(',')?;
                            }

                            write.encode(value, &matcher)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Behaviour {
    pub first: Option<char>,
    pub sep: char,
    pub named: bool,
    pub ifemp: Option<char>,
    pub allow: Allow,
}

#[derive(Debug)]
pub enum Allow {
    U,
    UR,
}

impl Allow {
    pub fn matcher(&self) -> Box<dyn Satisfy> {
        match self {
            Self::U => Box::new(satisfy::unreserved()),
            Self::UR => Box::new(satisfy::unreserved_or_reserved()),
        }
    }
}
