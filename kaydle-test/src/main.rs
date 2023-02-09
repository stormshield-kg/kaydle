use std::{
    collections::HashMap,
    io::{self, Read},
};

use anyhow::Context;
use kaydle::serde::de::from_str;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename = "item")]
struct Item(i32, i32, char);

#[derive(Deserialize, Debug)]
enum Enum {
    #[serde(rename = "int")]
    Int(i32),

    #[serde(rename = "string")]
    String(String),
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct AnnotatedString {
    #[serde(rename = "$kaydle::annotation")]
    annotation: Option<String>,
    value: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Document {
    name: String,
    age: i32,
    key_value: HashMap<String, i32>,
    items: Vec<Item>,
    enums: Vec<Enum>,
    annotated_values: Vec<AnnotatedString>,
}

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();

    io::stdin()
        .read_to_string(&mut buf)
        .context("Failed to read from stdin")?;

    let values: Document = from_str(&buf).context("Failed to deserialize")?;

    println!("{:#?}", values);

    Ok(())
}

#[cfg(test)]
mod test {
    use kaydle::serde::from_str;
    use serde::Deserialize;

    #[test]
    fn test_option_enum() -> anyhow::Result<()> {
        #[derive(Debug, PartialEq, Eq, Deserialize)]
        enum Enum {
            A,
            #[serde(rename = "C")]
            B,
        }

        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct Node {
            a: Option<Enum>,
            b: Option<Enum>,
            c: Option<Enum>,
            d: Option<Enum>,
        }

        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct Document {
            field0: Option<Node>,
            field1: Node,
            field2: Vec<Enum>,
            field3: Enum,
            node1: Option<String>,
            node2: Option<String>,
            node3: Option<String>,
        }

        let doc: Document = from_str(
            r#"
                field3 "C"
                field2 "A" "C"
                node3 null
                node1 "string"
                field1 a="A" b="C" d=null
            "#,
        )?;

        assert_eq!(
            doc,
            Document {
                field0: None,
                field1: Node {
                    a: Some(Enum::A),
                    b: Some(Enum::B),
                    c: None,
                    d: None,
                },
                field2: vec![Enum::A, Enum::B],
                field3: Enum::B,
                node1: Some("string".into()),
                node2: None,
                node3: None,
            }
        );

        Ok(())
    }
}
