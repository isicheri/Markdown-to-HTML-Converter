
use crate::token::{Node,State};


pub fn parser(input: &str) -> Vec<Node> {
 let line = input.trim();
 let mut nodes = Vec::new();

 for line in input.lines() {
 if line.starts_with("#") {
    let level = line.chars().take_while(|&c| c == '#').count();
    let text = line[level..].to_string();
    nodes.push(Node::Heading { level: level, text: text });
 }else if !line.is_empty() {
  let p_node = extract_from_paragraph(line.to_string());
  nodes.push(p_node);
 }

}

nodes

}


fn extract_from_paragraph(text: String) -> Node {

    let mut state = State::Normal;
    let mut buffer = String::new();
    let mut children = Vec::new(); 

    if(!text.contains("*")) {
        children.push(Node::Text(text.clone()));
      return Node::Paragraph(children);
    }

    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        if state == State::Normal {
            if ch == '*' && i + 1 < chars.len() && chars[i + 1] == '*' {
                children.push(Node::Text(buffer.clone()));
                buffer = "".to_string();
                state = State::Bold;
                i += 2; // Skip the next '*'
            }else if ch == '*' {
                children.push(Node::Text(buffer.clone()));
                buffer = "".to_string();
                state = State::Italic;
                i += 1;
            }else {
            buffer.push(ch);
            i += 1
            }
        }else if state == State::Italic {

            if ch == '*' {
           children.push(Node::Italic(buffer.clone()));
            buffer = "".to_string();
            state = State::Normal;
            i += 1;
            }else {
                buffer.push(ch);
                i += 1;
            }
            
        }else if state == State::Bold  {
            
             if ch == '*' && i + 1 < chars.len() && chars[i + 1] == '*' {
                children.push(Node::Bold(buffer.clone()));
                buffer = "".to_string();
                state = State::Normal;
                i += 1; // Skip the next '*'
            }else {
                buffer.push(ch);
                i += 1;
            }

        }

    }
    Node::Paragraph(children)

}