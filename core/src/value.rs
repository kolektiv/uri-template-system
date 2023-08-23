use std::collections::HashMap;

use fnv::FnvBuildHasher;

// =============================================================================
// Value
// =============================================================================

// Types

/// The `Values` type is used as the source of content during template
/// expansion, and is a logical map of keys to typed values (which may or may
/// not be present during expansion).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Values {
    values: HashMap<String, Value, FnvBuildHasher>,
}

impl Values {
    /// Adds a new `Value` to the `Values` collection and returns the modified
    /// collection to allow for chaining of calls during construction. Values
    /// may be any type which implements `Into<Value>` - this will generally be
    /// a concrete `Value` but may be your own type for which this trait has
    /// been implemented.
    ///
    /// For clarity, it may be better to implement a suitable iterator trait for
    /// your custom type and pass it to the relevant `Value` construction
    /// function, as this will make the shape of data produced more obvious for
    /// anyone reading the code.
    #[must_use]
    pub fn add(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.values.insert(key.into(), value.into());
        self
    }

    /// Gets the value at the given key from the `Values` collection if it
    /// exists.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }
}

impl FromIterator<(String, Value)> for Values {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        Self {
            values: HashMap::from_iter(iter),
        }
    }
}

/// The `Value` type is used as the source of content during template expansion,
/// as part of a `Values` collection. It maps to the three valid shapes of data
/// defined by the RFC (a single item, a list of items, or a list of key/value
/// pairs).
///
/// All values are `String`s for simplicity of ownership, etc.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    AssociativeArray(Vec<(String, String)>),
    Item(String),
    List(Vec<String>),
}

impl Value {
    /// Constructs a new `Value` from any iterator which produces pairs (tuples)
    /// where both items implement `Into<String>`. This may be a simple array or
    /// vec, or a more complex type such as an `IndexMap`.
    ///
    /// ```
    /// # use uri_template_system_core::Value;
    /// #
    /// let expected = Value::AssociativeArray(Vec::from_iter([
    ///     (String::from("a"), String::from("1")),
    ///     (String::from("b"), String::from("2")),
    /// ]));
    ///
    /// let array = [("a", "1"), ("b", "2")];
    /// assert_eq!(expected, Value::associative_array(array));
    ///
    /// let vec = Vec::from_iter(array);
    /// assert_eq!(expected, Value::associative_array(vec));
    /// ```
    pub fn associative_array<T, U, V>(value: T) -> Self
    where
        T: IntoIterator<Item = (U, V)>,
        U: Into<String>,
        V: Into<String>,
    {
        Self::AssociativeArray(
            value
                .into_iter()
                .map(|(u, v)| (u.into(), v.into()))
                .collect(),
        )
    }

    /// Constructs a new `Value` from any iterator which produces items which
    /// implement `Into<String>`, such as arrays, vecs, etc.
    ///
    /// ```
    /// # use uri_template_system_core::Value;
    /// #
    /// let expected = Value::List(Vec::from_iter([String::from("a"), String::from("b")]));
    ///
    /// let array = ["a", "b"];
    /// assert_eq!(expected, Value::list(array));
    ///
    /// let vec = Vec::from_iter(array);
    /// assert_eq!(expected, Value::list(vec));
    /// ```
    pub fn item<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::Item(value.into())
    }

    /// Constructs a new `Value` from any type which implements `Into<String>`.
    ///
    /// ```
    /// # use uri_template_system_core::Value;
    /// #
    /// let expected = Value::Item(String::from("a"));
    ///
    /// let str = "a";
    /// assert_eq!(expected, Value::item(str));
    ///
    /// let string = String::from(str);
    /// assert_eq!(expected, Value::item(string));
    /// ```
    pub fn list<T, U>(value: T) -> Self
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        Self::List(value.into_iter().map(Into::into).collect())
    }
}

impl Value {
    #[must_use]
    pub(crate) fn defined(&self) -> bool {
        match self {
            Self::AssociativeArray(value) if value.is_empty() => false,
            Self::List(value) if value.is_empty() => false,
            _ => true,
        }
    }
}

// -----------------------------------------------------------------------------

// Tests

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn associative_array_value_construction() {
        let expected = Value::AssociativeArray(Vec::from_iter([
            (String::from("a"), String::from("1")),
            (String::from("b"), String::from("2")),
        ]));

        let array = [("a", "1"), ("b", "2")];
        assert_eq!(expected, Value::associative_array(array));

        let vec = Vec::from_iter(array);
        assert_eq!(expected, Value::associative_array(vec));
    }

    #[test]
    fn item_value_construction() {
        let expected = Value::Item(String::from("a"));

        let str = "a";
        assert_eq!(expected, Value::item(str));

        let string = String::from(str);
        assert_eq!(expected, Value::item(string));
    }

    #[test]
    fn list_value_construction() {
        let expected = Value::List(Vec::from_iter([String::from("a"), String::from("b")]));

        let array = ["a", "b"];
        assert_eq!(expected, Value::list(array));

        let vec = Vec::from_iter(array);
        assert_eq!(expected, Value::list(vec));
    }
}
