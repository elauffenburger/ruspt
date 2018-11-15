extern crate rusptlib;

use std::io::{self, Read, Write};
use rusptlib::{parse, exec};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let program = parse(buffer);
        let result = exec(rusptlib::new_env(), program);

        println!("{:?}", result);
    }
}
