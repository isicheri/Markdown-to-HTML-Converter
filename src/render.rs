use crate::{render, token::Node};



pub fn render_html(nodes: Vec<Node>) -> String {
    let mut  html = String::new();

   for node in nodes  {
       match node {
        Node::Heading { level, text } => {
            html.push_str(format!("<h{lvl}>{}</h{lvl}>",text,lvl = level).as_str());
        },
        Node::Paragraph(nodes) => {
            for node in nodes  {
               
            }
        },
        Node::Bold(_) => todo!(),
        Node::Italic(_) => todo!(),
        Node::Text(_) => todo!(),
           }
   }

    html
}