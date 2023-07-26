// =============================================================================
// Util
// =============================================================================

// Predicates

// Generally useful value predicates for building up more complex predicate
// functions in parsers, etc.

pub fn is_equal_to<T>(a: T, b: T) -> bool
where
    T: Eq,
{
    a == b
}

pub fn is_in_range<T>(a: T, min: T, max: T) -> bool
where
    T: PartialOrd,
{
    a >= min && a <= max
}
