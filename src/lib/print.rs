use core::*;

use std::cell::RefCell;
use std::rc::Rc;

pub fn print(program: &LispProgram) -> String {
    match program.entry {
        None => "".to_string(),
        Some(ref entry) => print_cell(entry.clone()),
    }
}

pub fn print_cell(cell: LispCellRef) -> String {
    let mut result = String::new();
    print_rec(cell, &mut result);

    result
}

fn print_rec(node: LispCellRef, result: &mut String) {
    match *node.borrow() {
        LispCell::Quoted(ref quoted) => {
            print_rec(quoted.clone(), result);
            *result = format!("'{}", result);
        }
        LispCell::Number(num) => result.push_str(num.to_string().as_str()),
        LispCell::Atom(ref atom) => result.push_str(atom.as_str()),
        LispCell::List {
            ref contents,
        } => {
            result.push('(');

            let borrowed_contents = contents.borrow();
            let mut iter = borrowed_contents.iter().peekable();

            loop {
                match iter.next() {
                    Some(cell) => {
                        print_rec(cell.clone(), result);

                        match iter.peek() {
                            Some(_) => result.push(' '),
                            None => {}
                        };
                    }
                    None => break,
                };
            }

            result.push(')');
        }
        ref c @ _ => panic!("Unsupported LispCell type: {:?}", c),
    }
}
