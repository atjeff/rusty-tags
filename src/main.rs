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
                        let (tag_name, attributes) = extract_open_tag_info(&mut chars);

                        tokens.push(Token::OpenTag(tag_name, attributes));
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

// Extracts the tag name and attributes from the input
// This function assumes that the first character is a "<"
// Example input: <div class="container">
fn extract_open_tag_info(
    chars: &mut std::iter::Peekable<std::str::Chars>,
) -> (String, Vec<HtmlAttribute>) {
    let tag_name = extract_tag_name(chars);
    let mut attributes = Vec::new();

    while let Some(c) = chars.peek() {
        match c {
            // We reached the end of the tag, break
            '>' => {
                chars.next();
                break;
            }

            // We found an attribute. Example: ` class="container"`
            ' ' => {
                chars.next();

                let attribute = extract_attribute(chars);

                attributes.push(attribute);
            }

            _ => {
                chars.next();
            }
        }
    }

    (tag_name, attributes)
}

// Extracts attributes from the input
// This function assumes that the first character is a "<"
fn extract_attribute(chars: &mut std::iter::Peekable<std::str::Chars>) -> HtmlAttribute {
    let mut name = String::from("");
    let mut value = String::from("");

    // Loop through until we find '=', everything before that is the tag name
    while let Some(&c) = chars.peek() {
        // We're at the end of the tag name
        if c == '=' {
            chars.next();
            break;
        }

        name.push(chars.next().unwrap());
    }

    if let Some(&c) = chars.peek() {
        if c == '"' || c == '\'' {
            chars.next(); // skip the opening quote

            while let Some(&c) = chars.peek() {
                if c == '"' || c == '\'' {
                    chars.next(); // skip the closing quote
                    break;
                }

                value.push(chars.next().unwrap());
            }
        }
    }

    HtmlAttribute { name, value }
}

// Extracts the tag name from the input
// This function assumes that the first character is a "<"
fn extract_tag_name(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let skip_chars = vec![' ', '>', '/'];
    let mut tag_name = String::new();

    // Loop through the characters until we find a space, ">", or "/", return result (tag name)
    while let Some(c) = chars.peek() {
        if skip_chars.contains(&c) {
            break;
        }

        tag_name.push(chars.next().unwrap());
    }

    tag_name
}

fn main() {
    // Raw string without escaping quotes
    let example_input = r#"<div class="container"><p>Hello, world!</p></div>"#;
    // let example_input = r#"<div class="container"></div>"#;

    let tokens = tokenize(example_input).unwrap();

    println!("{:#?}", tokens);
}
