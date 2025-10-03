#![allow(unused)]

pub mod parser;
pub mod file; 
pub mod token;
pub mod render;
use crate::parser::parser;

fn main() {


      let md = r#"## Hello world
    this is the paragraph **hello** 
    the end *world*
    "#;

    let x = parser(md);

    println!("{:?}",x);

}