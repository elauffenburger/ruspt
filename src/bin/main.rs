extern crate rusptlib;

use rusptlib::{exec_prog, parse};
use std::io::{self, Read, Write};

fn main() {
    print!("Welcome to ruspt!");

    let mut env = rusptlib::new_env();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let program = parse(buffer);
        let result = exec_prog(&mut env, program);

        println!("{:?}", result);
    }
}
