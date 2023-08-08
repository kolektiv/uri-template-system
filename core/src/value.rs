use crate::IndexMap;

// =============================================================================
// Values
// =============================================================================

// Types

#[derive(Clone, Debug)]
pub struct Values {
    pub values: IndexMap<String, Value>,
}

impl Values {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    pub fn defined(&self) -> Self {
        Values::from_iter(self.values.iter().filter_map(|(key, value)| match value {
            Value::List(value) if value.is_empty() => None,
            Value::AssociativeArray(value) if value.is_empty() => None,
            value => Some((key.to_owned(), value.to_owned())),
        }))
    }
}

impl FromIterator<(String, Value)> for Values {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        Self {
            values: IndexMap::from_iter(iter),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Item(String),
    List(Vec<String>),
    AssociativeArray(IndexMap<String, String>),
}
