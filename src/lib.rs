use std::fs::File;
use std::io::prelude::*;
mod vm;
mod parser;

// Returns file contents as 'String' buffer
fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}
// Interpret and run a bf file
pub fn bf_interpret(filename: &str) {
    let contents = read_file(filename);
    let mut machine = vm::BFMachine::new();
    machine.process(parser::parse_src(contents));
}
