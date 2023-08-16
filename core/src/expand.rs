use std::fmt::{
    Display,
    Formatter,
    Result,
    Write,
};

use crate::{
    encode::{
        self,
        EncodeExt,
    },
    satisfy::{
        Ascii,
        PercentEncoded,
        Satisfier,
    },
    Component,
    Expression,
    Literal,
    Modifier,
    OpLevel2,
    OpLevel3,
    Operator,
    Template,
    Value,
    Values,
};

// =============================================================================
// Expansion
// =============================================================================

// Traits

pub trait Expand {
    fn expand(&self, values: &Values, f: &mut Formatter<'_>) -> Result;
}

// -----------------------------------------------------------------------------

// Types

pub struct Expansion<'e, 't> {
    template: &'e Template<'t>,
    values: &'e Values,
}

impl<'e, 't> Expansion<'e, 't> {
    pub fn new(template: &'e Template<'t>, values: &'e Values) -> Self {
        Self { template, values }
    }
}

impl<'e, 't> Display for Expansion<'e, 't> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.template.expand(self.values, f)
    }
}

// =============================================================================
// Implementations
// =============================================================================

// Template

impl<'t> Expand for Template<'t> {
    fn expand(&self, values: &Values, f: &mut Formatter<'_>) -> Result {
        self.components
            .iter()
            .try_for_each(|component| component.expand(values, f))
    }
}

// -----------------------------------------------------------------------------

// Component

impl<'t> Expand for Component<'t> {
    fn expand(&self, values: &Values, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Expression(expression) => expression.expand(values, f),
            Self::Literal(literal) => literal.expand(values, f),
        }
    }
}

// -----------------------------------------------------------------------------

// Expression

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
    pub fn matcher(&self) -> Box<dyn Satisfier> {
        match self {
            Self::U => return Box::new(Ascii::new(|b| encode::is_unreserved_ascii(b))),
            Self::UR => {
                return Box::new((
                    Ascii::new(|b| encode::is_unreserved_ascii(b) || encode::is_reserved_ascii(b)),
                    PercentEncoded,
                ));
            }
        }
    }
}

impl<'t> Expand for Expression<'t> {
    fn expand(&self, values: &Values, f: &mut Formatter<'_>) -> Result {
        let behaviour = self
            .operator
            .as_ref()
            .map(|operator| operator.behaviour())
            .unwrap_or(&DEFAULT_BEHAVIOUR);

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
                    f.write_char(c)?;
                }

                first = false;
            } else {
                f.write_char(behaviour.sep)?;
            }

            if let Value::Item(value) = value {
                // If this variable's value is a string, then

                if behaviour.named {
                    // * if named is true, append the varname to the result string using the same
                    //   encoding process as for literals, and

                    f.write_str_encoded(var_name.name(), &Literal::expansion())?;

                    if value.is_empty() {
                        // + if the value is empty, append the ifemp string to the result string and
                        //   skip to the next varspec;

                        if let Some(c) = behaviour.ifemp {
                            f.write_char(c)?;
                        }
                    } else {
                        // + otherwise, append "=" to the result string.

                        f.write_char('=')?;
                    }
                }

                match modifier {
                    Some(Modifier::Prefix(prefix)) => {
                        // * if a prefix modifier is present and the prefix length is less than the
                        //   value string length in number of Unicode characters, append that number
                        //   of characters from the beginning of the value string to the result
                        //   string, after pct-encoding any characters that are not in the allow
                        //   set, while taking care not to split multi-octet or pct-encoded triplet
                        //   characters that represent a single Unicode code point;

                        let pos: usize = value
                            .chars()
                            .take(prefix.length())
                            .map(|c| c.len_utf8())
                            .sum();

                        f.write_str_encoded(&value[..pos], &matcher)?;
                    }
                    _ => {
                        // * otherwise, append the value to the result string after pct-encoding any
                        //   characters that are not in the allow set.

                        f.write_str_encoded(value, &matcher)?;
                    }
                };
            } else if let Some(Modifier::Explode(_)) = modifier {
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
                                f.write_char(behaviour.sep)?;
                            }

                            // + if this is a pair, append the name to the result string using the
                            //   same encoding process as for literals;

                            f.write_str_encoded(name, &Literal::expansion())?;

                            // + if the member/value is empty, append the ifemp string to the result
                            //   string; otherwise, append "=" and the member/value to the result
                            //   string, after pct-encoding any member/value characters that are not
                            //   in the allow set.

                            if value.is_empty() {
                                if let Some(c) = behaviour.ifemp {
                                    f.write_char(c)?;
                                }
                            } else {
                                f.write_char('=')?;
                                f.write_str_encoded(value, &matcher)?;
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
                                f.write_char(behaviour.sep)?;
                            }

                            // + if this is a list, append the varname to the result string using
                            //   the same encoding process as for literals;

                            f.write_str_encoded(var_name.name(), &Literal::expansion())?;

                            // + if the member/value is empty, append the ifemp string to the result
                            //   string; otherwise, append "=" and the member/value to the result
                            //   string, after pct-encoding any member/value characters that are not
                            //   in the allow set.

                            if value.is_empty() {
                                if let Some(c) = behaviour.ifemp {
                                    f.write_char(c)?;
                                }
                            } else {
                                f.write_char('=')?;
                                f.write_str_encoded(value, &matcher)?;
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
                                    f.write_char(behaviour.sep)?;
                                }
                            }

                            f.write_str_encoded(name, &matcher)?;
                            f.write_char('=')?;
                            f.write_str_encoded(value, &matcher)?;
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
                                    f.write_char(behaviour.sep)?;
                                }
                            }

                            f.write_str_encoded(value, &matcher)?;
                        }
                    }
                }
            } else {
                // else if no explode modifier is given, then

                if behaviour.named {
                    // * if named is true, append the varname to the result string using the same
                    //   encoding process as for literals, and

                    f.write_str_encoded(var_name.name(), &Literal::expansion())?;

                    // + if the value is empty, append the ifemp string to the result string and
                    //   skip to the next varspec;
                    // + otherwise, append "=" to the result string; and

                    // NOTE: Empty values are not meaningful currently, so this logic is skipped for
                    // now

                    f.write_char('=')?;
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
                                f.write_char(',')?;
                            }

                            f.write_str_encoded(name, &matcher)?;
                            f.write_char(',')?;
                            f.write_str_encoded(value, &matcher)?;
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
                                f.write_char(',')?;
                            }

                            f.write_str_encoded(value, &matcher)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------

// Operator

impl<'t> Operator<'t> {
    pub fn behaviour(&self) -> &Behaviour {
        match self {
            Self::Level2(op_level_2) => match op_level_2 {
                OpLevel2::Fragment(_) => &FRAGMENT_BEHAVIOUR,
                OpLevel2::Reserved(_) => &RESERVED_BEHAVIOUR,
            },
            Self::Level3(op_level_3) => match op_level_3 {
                OpLevel3::Label(_) => &LABEL_BEHAVIOUR,
                OpLevel3::Path(_) => &PATH_BEHAVIOUR,
                OpLevel3::PathParameter(_) => &PATH_PARAMETER_BEHAVIOUR,
                OpLevel3::Query(_) => &QUERY_BEHAVIOUR,
                OpLevel3::QueryContinuation(_) => &QUERY_CONTINUATION_BEHAVIOUR,
            },
        }
    }
}

macro_rules! behaviour {
    ($name:ident, $first:stmt, $sep:literal, $named:literal, $ifemp:stmt, $allow:ty) => {
        paste::paste! {
            static [< $name:snake:upper _BEHAVIOUR >]: Behaviour = Behaviour {
                first: $first,
                sep: $sep,
                named: $named,
                ifemp: $ifemp,
                allow: $allow
            };
        }
    };
}
// Operator - None

behaviour!(Default, None, ',', false, None, Allow::U);

// Operator - Level 2

behaviour!(Fragment, Some('#'), ',', false, None, Allow::UR);
behaviour!(Reserved, None, ',', false, None, Allow::UR);

// Operator - Level 3

behaviour!(Label, Some('.'), '.', false, None, Allow::U);
behaviour!(Path, Some('/'), '/', false, None, Allow::U);
behaviour!(PathParameter, Some(';'), ';', true, None, Allow::U);
behaviour!(Query, Some('?'), '&', true, Some('='), Allow::U);
behaviour!(QueryContinuation, Some('&'), '&', true, Some('='), Allow::U);

// -----------------------------------------------------------------------------

// Literal

impl<'t> Literal<'t> {
    pub const fn expansion() -> impl Satisfier {
        (
            Ascii::new(|b| encode::is_unreserved_ascii(b) || encode::is_reserved_ascii(b)),
            PercentEncoded,
        )
    }
}

impl<'t> Expand for Literal<'t> {
    fn expand(&self, _values: &Values, f: &mut Formatter<'_>) -> Result {
        f.write_str_encoded(self.raw, &Self::expansion())
    }
}
