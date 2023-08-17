use anyhow::Result;

// =============================================================================
// Parse
// =============================================================================

// Traits

pub trait Parse<'t>
where
    Self: Sized,
{
    fn parse(raw: &'t str) -> (usize, Self);
}

pub trait TryParse<'t>
where
    Self: Sized,
{
    fn try_parse(raw: &'t str) -> Result<(usize, Self)>;
}
