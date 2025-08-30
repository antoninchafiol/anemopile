mod lexer;

use std::{fs::File, io::Read};
use std::env;

fn main() {
    
    println!("{}", env::current_dir().unwrap().display());

    let mut f = File::open("test.bas").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content);

    let content_vec: Vec<&str> = content.split("\n").collect();
    for i in content_vec {
        println!("- {}",i);
    }
    lexer::test();
}
