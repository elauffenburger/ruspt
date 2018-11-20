extern crate rusptlib;

use rusptlib::{exec_prog, parse, Environment};
use std::env;
use std::io::{self, Write};

enum RunMode {
    Repl,
    Server,
}

fn main() {
    let mut args = env::args().skip(1);
    let mut run_mode = None;

    loop {
        let arg = args.next();

        match arg {
            Some(arg) => match arg.as_str() {
                "--repl" => run_mode = Some(RunMode::Repl),
                "--server" => run_mode = Some(RunMode::Server),
                _ => panic!("Unknown option {:?}", arg),
            },
            None => break,
        };
    }

    match run_mode {
        None | Some(RunMode::Repl) => repl(),
        Some(RunMode::Server) => server(),
    }
}

fn repl() {
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

fn server() {
    print!("Starting server...");
}
