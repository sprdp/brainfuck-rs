# BFRS
A simple brainfuck interpreter writter in rust. \
Currently only supports interpreting input as a file. Program input is to stdin and output to stdout. 


## Usage
### As a binary
    bfrs <Name of file>

### As a library
```rust
extern crate bfrs;
bfrs::bf_interpret("filepath");
```

