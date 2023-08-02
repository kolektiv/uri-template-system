use indexmap::IndexMap;

// =============================================================================
// Values
// =============================================================================

// Types

#[derive(Debug)]
pub struct Values {
    pub values: IndexMap<String, Value>,
}

impl Values {
    pub fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (String, Value)>,
    {
        Self {
            values: IndexMap::from_iter(iter),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    pub fn defined(&self) -> Values {
        Values::from_iter(self.values.iter().filter_map(|(key, value)| match value {
            Value::List(value) if value.is_empty() => None,
            Value::AssociativeArray(value) if value.is_empty() => None,
            value => Some((key.to_owned(), value.to_owned())),
        }))
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Item(String),
    List(Vec<String>),
    AssociativeArray(IndexMap<String, String>),
}
