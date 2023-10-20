use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article = Article {
        article: String::from("How to work with json in rust"),
        author: String::from("Mayank Singh"),
        paragraph: vec![
            Paragraph {
                name: String::from("First Sentence"),
            },
            Paragraph {
                name: String::from("Second Sentence"),
            },
            Paragraph {
                name: String::from("Third Sentence"),
            },
        ],
    };

    let json = serde_json::to_string_pretty(&article).unwrap();

    println!("The json is {:?}", json)
}
