use rusptlib::{exec_prog, parse, Environment};
use std::env;
use std::io::{self, Write};

pub fn repl() {
    println!("Welcome to ruspt!");

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