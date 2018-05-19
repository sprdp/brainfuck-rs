use std::env;
use std::fs::File;
use std::io::prelude::*;

// Returns file contents as 'String' buffer
fn read_file(filename: &str) -> String {
    println!("{}", filename);
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: bfrs <filename>");
    } else {
        let src_name = &args[1];
        let contents = read_file(src_name);
        println!("{}", contents);
    }
}
