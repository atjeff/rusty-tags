use crate::{
    html_element::{HtmlAttribute, HtmlElement},
    tokenizer::{HtmlParseError, Token},
};

// Lets define a trait that any struct? needs to implement
pub trait Parse {
    fn parse(input: &str) -> Result<HtmlElement, HtmlParseError>;
}

pub fn parse_tokens(
    tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>,
    parent: &mut HtmlElement,
) -> Result<(), HtmlParseError> {
    while let Some(token) = tokens.next() {
        match token {
            Token::OpenTag(tag_name, attributes) => {
                // We found an opening tag, so we need to create a new element
                let mut element = HtmlElement {
                    tag_name: tag_name.clone(),
                    attributes: attributes,
                    children: Vec::new(),
                };

                // We need to recursively parse the children of this element
                parse_tokens(tokens, &mut element)?;

                parent.children.push(element);
            }
            Token::Text(text) => {
                let text_node = HtmlElement {
                    tag_name: "#text".to_string(),
                    attributes: vec![HtmlAttribute {
                        name: "content".to_string(),
                        value: text,
                    }],
                    children: Vec::new(),
                };

                parent.children.push(text_node);
            }
            Token::CloseTag(tag_name) => {
                if parent.tag_name == tag_name {
                    return Ok(());
                } else {
                    return Err(HtmlParseError::MismatchedTag {
                        expected: parent.tag_name.clone(),
                        found: tag_name,
                    });
                }
            }
        }
    }

    Ok(())
}
