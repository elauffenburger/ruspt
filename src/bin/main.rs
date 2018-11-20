extern crate rusptlib;

use rusptlib::{exec_prog, parse, Environment};
use std::io::{self, Write};

fn main() {
    print!("Welcome to ruspt!");

    let mut env = Environment::new();

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
