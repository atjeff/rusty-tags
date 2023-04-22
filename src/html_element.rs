use crate::{
    parser::{parse_tokens, Parse},
    tokenizer::{tokenize, HtmlParseError},
};

#[derive(Debug)]
pub struct HtmlElement {
    pub tag_name: String,
    pub attributes: Vec<HtmlAttribute>,
    pub children: Vec<HtmlElement>,
}

#[derive(Debug)]
pub struct HtmlAttribute {
    pub name: String,
    pub value: String,
}

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
