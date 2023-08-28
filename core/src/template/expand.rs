use std::fmt::{
    Error,
    Write,
};

use thiserror::Error;

use crate::{
    string::{
        encode::Encode,
        satisfy::{
            self,
            Satisfy,
        },
    },
    template::{
        Component,
        Expression,
        Literal,
        Modifier,
        OpLevel2,
        OpLevel3,
        Operator,
        Template,
    },
    value::{
        Value,
        Values,
    },
};

// =============================================================================
// Expand
// =============================================================================

// Traits

#[allow(clippy::module_name_repetitions)]
pub trait Expand {
    fn expand(&self, values: &Values, write: &mut impl Write) -> Result<(), ExpandError>;
}

// -----------------------------------------------------------------------------

// Errors

/// An [`Error`](std::error::Error) compatible type which may be the result of a
/// failure of [`Template::expand`] (given a valid [`Template`] and provided
/// [`Values`]).
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error)]
pub enum ExpandError {
    /// Formatting for this expansion failed due to an internal error in
    /// [`std::fmt::Write`], which is not recoverable.
    #[error("formatting failed")]
    Format(#[from] Error),
}

// =============================================================================
// Expand - Implementations
// =============================================================================

// Template

impl<'t> Expand for Template<'t> {
    fn expand(&self, values: &Values, write: &mut impl Write) -> Result<(), ExpandError> {
        self.components
            .iter()
            .try_for_each(|component| component.expand(values, write))
    }
}

// -----------------------------------------------------------------------------

// Component

impl<'t> Expand for Component<'t> {
    fn expand(&self, values: &Values, write: &mut impl Write) -> Result<(), ExpandError> {
        match self {
            Self::Expression(expression) => expression.expand(values, write),
            Self::Literal(literal) => literal.expand(values, write),
        }
    }
}

// -----------------------------------------------------------------------------

// Expression

impl<'t> Expand for Expression<'t> {
    #[allow(clippy::cognitive_complexity)] // TODO: Reduce?
    #[allow(clippy::equatable_if_let)]
    #[allow(clippy::too_many_lines)]
    fn expand(&self, values: &Values, write: &mut impl Write) -> Result<(), ExpandError> {
        let behaviour = self
            .operator
            .as_ref()
            .map_or(&DEFAULT_BEHAVIOUR, Operator::behaviour);

        let satisfier = behaviour.allow.satisfier();
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

                        write.encode(&value[..pos], &satisfier)?;
                    }
                    _ => {
                        // * otherwise, append the value to the result string after pct-encoding any
                        //   characters that are not in the allow set.

                        write.encode(value, &satisfier)?;
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
                                write.encode(value, &satisfier)?;
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
                                write.encode(value, &satisfier)?;
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

                            write.encode(name, &satisfier)?;
                            write.write_char('=')?;
                            write.encode(value, &satisfier)?;
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

                            write.encode(value, &satisfier)?;
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

                            write.encode(name, &satisfier)?;
                            write.write_char(',')?;
                            write.encode(value, &satisfier)?;
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

                            write.encode(value, &satisfier)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Behaviour {
    pub first: Option<char>,
    pub sep: char,
    pub named: bool,
    pub ifemp: Option<char>,
    pub allow: Allow,
}

#[derive(Debug)]
enum Allow {
    U,
    UR,
}

impl Allow {
    fn satisfier(&self) -> Box<dyn Satisfy> {
        match self {
            Self::U => Box::new(satisfy::unreserved()),
            Self::UR => Box::new(satisfy::unreserved_or_reserved()),
        }
    }
}

// -----------------------------------------------------------------------------

// Operator

impl Operator {
    fn behaviour(&self) -> &Behaviour {
        match self {
            Self::Level2(op_level_2) => match op_level_2 {
                OpLevel2::Fragment => &FRAGMENT_BEHAVIOUR,
                OpLevel2::Reserved => &RESERVED_BEHAVIOUR,
            },
            Self::Level3(op_level_3) => match op_level_3 {
                OpLevel3::Label => &LABEL_BEHAVIOUR,
                OpLevel3::Path => &PATH_BEHAVIOUR,
                OpLevel3::PathParameter => &PATH_PARAMETER_BEHAVIOUR,
                OpLevel3::Query => &QUERY_BEHAVIOUR,
                OpLevel3::QueryContinuation => &QUERY_CONTINUATION_BEHAVIOUR,
            },
        }
    }
}

static DEFAULT_BEHAVIOUR: Behaviour = Behaviour {
    first: None,
    sep: ',',
    named: false,
    ifemp: None,
    allow: Allow::U,
};

static FRAGMENT_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('#'),
    sep: ',',
    named: false,
    ifemp: None,
    allow: Allow::UR,
};

static RESERVED_BEHAVIOUR: Behaviour = Behaviour {
    first: None,
    sep: ',',
    named: false,
    ifemp: None,
    allow: Allow::UR,
};

static LABEL_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('.'),
    sep: '.',
    named: false,
    ifemp: None,
    allow: Allow::U,
};

static PATH_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('/'),
    sep: '/',
    named: false,
    ifemp: None,
    allow: Allow::U,
};

static PATH_PARAMETER_BEHAVIOUR: Behaviour = Behaviour {
    first: Some(';'),
    sep: ';',
    named: true,
    ifemp: None,
    allow: Allow::U,
};

static QUERY_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('?'),
    sep: '&',
    named: true,
    ifemp: Some('='),
    allow: Allow::U,
};

static QUERY_CONTINUATION_BEHAVIOUR: Behaviour = Behaviour {
    first: Some('&'),
    sep: '&',
    named: true,
    ifemp: Some('='),
    allow: Allow::U,
};

// -----------------------------------------------------------------------------

// Literal

impl<'t> Expand for Literal<'t> {
    fn expand(&self, _values: &Values, write: &mut impl Write) -> Result<(), ExpandError> {
        write.encode(self.value, &satisfy::unreserved_or_reserved())?;

        Ok(())
    }
}
