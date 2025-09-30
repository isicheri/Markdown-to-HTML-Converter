#![allow(unused)]
mod parser;
mod file; 

#[derive(Debug)]
enum AstNode {
    Header { level: usize, text: String },
    Paragraph(String)
}


fn tokenizer(input: &str) -> Vec<AstNode> {
    let mut node:Vec<AstNode> = Vec::new();

    for line in input.lines() {
        
      if line.starts_with("#") {
    // first we count the number of # in the line 
    let level = line.trim().chars().take_while(|c| *c == '#').count();

    // second we extract the text from the line
    let text = line[level..].to_string();

    // third we create the struct the push it to the node
    
    node.push(AstNode::Header { level: level, text: text });

      }else if !line.is_empty() {
             node.push(AstNode::Paragraph(line.to_string()));
      }
    }

    node
}


fn parser(nodes: Vec<AstNode>) -> String {
    let mut  html = String::new();
    
    for node in nodes.iter() {
        
        match node {
            AstNode::Header { level, text } => {
                html.push_str(format!("<h{0}>{1}</h{0}>",level,text).as_str());
            },
            AstNode::Paragraph(s) => {
                html.push_str(format!("<p>{}</p>",s).as_str());
            },
        }

    }

    html
}

fn main() {

    let x = "## hello world
    this is the paragraph";

    let token_node = tokenizer(x);
    let parsed_string = parser(token_node);

   println!("Html: {}",parsed_string);
}