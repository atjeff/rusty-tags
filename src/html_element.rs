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
