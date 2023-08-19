use std::error::Error;

use uri_template_system_core::{
    Template,
    Value,
    Values,
};

fn main() -> Result<(), Box<dyn Error>> {
    let template = Template::parse("/hello/{name}{/library}")?;
    let values = Values::default()
        .add("name", Value::item("world"))
        .add("library", Value::list(["uri", "template", "system"]));

    assert_eq!(
        template.expand(&values)?,
        "/hello/world/uri/template/system"
    );

    Ok(())
}
