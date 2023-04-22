mod html_element;
mod parser;
mod tokenizer;

use html_element::HtmlElement;
use parser::{parse_tokens, Parse};
use tokenizer::{tokenize, HtmlParseError};

impl Parse for HtmlElement {
    fn parse(input: &str) -> Result<HtmlElement, HtmlParseError> {
        let tokens = tokenize(input).unwrap();

        let mut root = HtmlElement {
            tag_name: String::new(),
            attributes: Vec::new(),
            children: Vec::new(),
        };

        parse_tokens(&mut tokens.into_iter().peekable(), &mut root)?;

        Ok(root.children.pop().unwrap())
    }
}

fn main() {
    // Raw string without escaping quotes
    let example_input = r#"<div class="container"><p>Hello, world!</p></div>"#;
    // let example_input = r#"<div class="container"></div>"#;

    match HtmlElement::parse(&example_input) {
        Ok(parsed_html) => println!("Parsed HTML: {:#?}", parsed_html),
        Err(err) => println!("Error parsing HTML: {:#?}", err),
    }
}
