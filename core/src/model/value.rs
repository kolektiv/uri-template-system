use std::collections::HashMap;

use fnv::FnvBuildHasher;

// =============================================================================
// Value
// =============================================================================

// Types

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug)]
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
