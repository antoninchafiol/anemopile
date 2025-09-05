mod lexer;

use std::{fs::File, io::Read};
use std::env;

fn main() {
    
    let mut f = File::open("test.bas").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content);

    let content_vec: Vec<String> = content
        .split("\n")
        .map(|f| f.to_string())
        .collect();

    for i in &content_vec {
        println!("- {}",i);
    }

    let res_vec = lexer::Lexer::tokenize(content_vec).unwrap();
    println!("{:?}", res_vec);
}
