use uri_template_system_core::{
    URITemplate,
    Value,
    Values,
};

fn main() {
    let template = URITemplate::parse("/literal/{simple}/{/expanded*}").unwrap();
    let values = Values::from_iter([
        ("simple".into(), Value::Item("hello".into())),
        (
            "expanded".into(),
            Value::List(vec!["world1".into(), "world2".into(), "world3".into()]),
        ),
    ]);

    let result = template.expand(&values);

    println!("{result}");
}
