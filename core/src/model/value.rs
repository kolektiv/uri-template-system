use std::collections::HashMap;

use fnv::FnvBuildHasher;

// =============================================================================
// Value
// =============================================================================

// Types

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Values {
    pub values: HashMap<String, Value, FnvBuildHasher>,
}

impl Values {
    #[must_use]
    pub fn add(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.values.insert(key.into(), value.into());
        self
    }

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    AssociativeArray(Vec<(String, String)>),
    Item(String),
    List(Vec<String>),
}

impl Value {
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

    pub fn item<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::Item(value.into())
    }

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
    pub fn defined(&self) -> bool {
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

        // NOTE: We reverse the array here as hash_map does not guarantee ordering,
        // though in practice it is generally LIFO. This is not recommended usage, but
        // illustrative - for reliable usage of a map type, use IndexMap.
        let hash_map: HashMap<&str, &str> = HashMap::from_iter(array.into_iter().rev());
        assert_eq!(expected, Value::associative_array(hash_map));
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
