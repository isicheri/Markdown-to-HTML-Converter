

#[derive(Debug)]
pub enum Node {
    Heading { level: usize, text: String },
    Paragraph(Vec<Node>),
    Bold(String),
    Italic(String),
    Text(String)   
}

#[derive(PartialEq)]
pub enum State {
    Normal,
    Bold,
    Italic
}