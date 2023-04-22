// Basic html element
#[derive(Debug)]
struct HtmlElement {
    tag_name: String,
    attributes: Vec<HtmlAttribute>,
    children: Vec<HtmlElement>,
}

// Basic html attribute
#[derive(Debug)]
struct HtmlAttribute {
    name: String,
    value: String,
}

// Lets define a trait that any struct? needs to implement
trait Parser {
    fn parse(&self, input: &str) -> Result<HtmlElement, String>;
}

#[derive(Debug)]
enum Token {
    Text(String),
    OpenTag(String, Vec<HtmlAttribute>),
    CloseTag(String),
}

#[derive(Debug)]
enum HtmlParseError {
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
}

fn tokenize(input: &str) -> Result<Vec<Token>, HtmlParseError> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            // If the character is a "<" then we're opening an HTML tag
            '<' => {
                // If the buffer is not empty, then we have some text that we need to add to the tokens
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }

                match chars.peek() {
                    // If the next character is a "/", then we're closing a tag
                    Some('/') => {
                        chars.next();
                        let tag_name = extract_tag_name(&mut chars);
                        tokens.push(Token::CloseTag(tag_name));
                    }

                    // Otherwise, we're opening a tag
                    Some(_) => {
                        let (tag_name, _attributes) = extract_open_tag_info(&mut chars);
                        tokens.push(Token::OpenTag(tag_name, vec![]));
                    }

                    // If we don't have a next character, then we have an unexpected end of input
                    None => return Err(HtmlParseError::UnexpectedEndOfInput),
                }
            }
            // If the character is a ">", then we're closing an HTML tag
            '>' => {}

            // Otherwise, we're just adding the character to the buffer
            _ => buffer.push(c),
        }
    }

    // If the buffer is not empty, then we have some text that we need to add to the tokens
    if !buffer.is_empty() {
        tokens.push(Token::Text(buffer));
    }

    // Return the tokens
    Ok(tokens)
}

fn extract_open_tag_info(
    chars: &mut std::iter::Peekable<std::str::Chars>,
) -> (String, Vec<HtmlAttribute>) {
    let tag_name = extract_tag_name(chars);
    let attributes = vec![];

    (tag_name, attributes)
}

// Extracts the tag name from the input
// This function assumes that the first character is a "<"
fn extract_tag_name(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let skip_chars = vec![' ', '>', '/'];
    let mut tag_name = String::new();

    // Loop through the characters until we find a space, ">", or "/", return result (tag name)
    while let Some(c) = chars.next() {
        if skip_chars.contains(&c) {
            break;
        }

        tag_name.push(c);
    }

    tag_name
}

fn main() {
    // Raw string without escaping quotes
    let example_input = r#"<div class="container"><p>Hello, world!</p></div>"#;

    let tokens = tokenize(example_input).unwrap();

    println!("Tokens: {:?}", tokens);
}
