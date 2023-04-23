mod html_element;
mod parser;
mod tokenizer;

use env_logger;
use html_element::HtmlElement;
use parser::Parse;

fn main() {
    env_logger::init();

    // Raw string without escaping quotes
    let test_cases = vec![
        r#"<div class="container"><p>Hello, world!</p></div>"#,
        r#"<div class="container"></div>"#,
        r#"
            <div class="container">
                <span>test123</span>
                <a href="test"><button><span class="text-bold">span inside button inside anchor</span></button></a>
            </div>
        "#,
        r#"
            <nav class="header__nav">
                <div class="header__left">
                </div>
                <div class="header__divider header__color--centennial-silver"></div>
                <div class="header__divider header__color--diamondring-blue"></div>
                <div class="header__divider header__color--fortknox-gold"></div>
                <div class="header__divider header__color--benjamins-green"></div>
                <div class="header__divider"></div>
                <div class="header__divider"></div>
                <div class="header__divider header__color--capitalist-teal"></div>
                <div class="header__divider header__color--statuesque-bronze"></div>
                <div class="header__divider header__color--merlot-burgundy"></div>
                <div class="header__divider"></div>
                <div class="header__divider"></div>
                <div class="header__divider header__color--shopping-salmon"></div>
                <div class="header__divider"></div>
                <div class="header__divider header__color--advisor-amethyst"></div>
                <div class="header__divider header__color--insights-blue"></div>
                <div class="header__divider header__color--default-gray"></div>
                <div class="header__divider header__color--opulent-purple"></div>
                <div class="header__divider header__color--default-gray"></div>
                <div class="header__divider"></div>
                <div class="header__divider header__color--merlot-burgundy"></div>
                <div class="header__divider header__color--default-gray"></div>
                <div class="header__divider header__color--default-gray"></div>
            </nav>
        "#,
    ];

    // loop through test_cases
    for test_case in test_cases {
        match HtmlElement::parse(&test_case) {
            Ok(parsed_html) => println!("Parsed HTML: {:#?}", parsed_html),
            Err(err) => println!("Error parsing HTML: {:#?}", err),
        }
    }
}
