extern crate rusptlib;

use rusptlib::{parse, print};

static _program: &'static str = "
    (do (print (+ 1 2) (- 1 2)) (foo bar baz (qux (+ 1 2) (blah) blah)))
";

fn main() {
    let program = parse(_program.to_string());
    println!("program: {:?}", &program);

    let printed_program = print(&program);
    println!("printed program: {}", &printed_program);

    assert!(
        program.text == printed_program,
        "Expected printed program to match original program"
    );

    println!("printed parsed program matches original program!");
}
