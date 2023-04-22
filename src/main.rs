mod html_element;
mod parser;
mod tokenizer;

use html_element::HtmlElement;
use parser::Parse;

fn main() {
    // Raw string without escaping quotes
    let test_cases = vec![
        r#"<div class="container"><p>Hello, world!</p></div>"#,
        r#"<div class="container"></div>"#,
    ];

    // loop through test_cases
    for test_case in test_cases {
        match HtmlElement::parse(&test_case) {
            Ok(parsed_html) => println!("Parsed HTML: {:#?}", parsed_html),
            Err(err) => println!("Error parsing HTML: {:#?}", err),
        }
    }
}
