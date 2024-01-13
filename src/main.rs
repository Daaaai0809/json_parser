mod lexer;
mod token;
mod util;
mod parser;
mod node;

fn main() {
    let input = r#"
        {
            "foo": "bar",
            "baz": 123,
            "qux": true,
            "quux": false,
            "corge": null,
            "grault": {
                "garply": "waldo",
                "fred": 42,
                "plugh": true,
                "xyzzy": false,
                "thud": null,
                "emptyObject": {},
                "emptyArray": []
            },
            "emptyObject": {},
            "emptyArray": []
        }
    "#;
    let mut parser = parser::Parser::new(input);
    let node = parser.parse();
    println!("{:#?}", node);
}
