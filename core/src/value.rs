use std::collections::HashMap;

// =============================================================================
// Values
// =============================================================================

// Types

#[derive(Debug)]
pub struct Values {
    pub values: HashMap<String, Value>,
}

impl Values {
    pub fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (String, Value)>,
    {
        Self {
            values: HashMap::from_iter(iter),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Item(String),
    List(Vec<String>),
    AssociativeArray(HashMap<String, String>),
}
