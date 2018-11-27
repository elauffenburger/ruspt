use std::env;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, Write};

use rusptlib::{exec_prog, parse, Environment};

pub fn repl() {
    println!("Welcome to ruspt!");

    let mut env = Rc::new(RefCell::new(Environment::new()));

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let program = parse(buffer);
        let result = exec_prog(env.clone(), program);

        println!("{:?}", result);
    }
}