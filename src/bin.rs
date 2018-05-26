extern crate bfrs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: bfrs <filename>");
    } else {
        let src_name = &args[1];
        bfrs::bf_interpret(src_name);
    }
}
