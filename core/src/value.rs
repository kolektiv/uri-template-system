use std::collections::HashMap;

use fnv::FnvBuildHasher;

// =============================================================================
// Values
// =============================================================================

// Types

#[derive(Clone, Debug)]
pub struct Values {
    pub values: HashMap<String, Value, FnvBuildHasher>,
}

impl Values {
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
    pub fn defined(&self) -> bool {
        match self {
            Self::AssociativeArray(value) if value.is_empty() => false,
            Self::List(value) if value.is_empty() => false,
            _ => true,
        }
    }
}
