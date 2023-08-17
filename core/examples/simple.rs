use uri_template_system_core::{
    Template,
    Value,
    Values,
};

fn main() {
    let template = Template::parse("/literal/{simple}{/expanded*}").unwrap();
    let values = Values::from_iter([
        ("simple".into(), Value::Item("hello".into())),
        (
            "expanded".into(),
            Value::List(vec!["world1".into(), "world 2".into(), "world3".into()]),
        ),
    ]);

    let result = template.expand(&values);

    println!("{result}");
}
