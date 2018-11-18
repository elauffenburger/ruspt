use core::*;

use std::rc::Rc;
use std::cell::RefCell;

pub fn print(program: &LispProgram) -> String {
    match program.entry {
        None => "".to_string(),
        Some(ref entry) => {
            let mut result = String::new();
            print_rec(entry.clone(), &mut result);

            return result;
        }
    }
}

fn print_rec(node: Rc<RefCell<LispCell>>, result: &mut String) {
    match *node.borrow() {
        LispCell::Number(num) => result.push_str(num.to_string().as_str()),
        LispCell::Atom(ref atom) => result.push_str(atom.as_str()),
        LispCell::List {
            ref contents,
        } => {
            result.push('(');

            let n = contents.len();
            for i in 0..n {
                let cell = &contents[i];

                print_rec(cell.clone(), result);

                if i != n - 1 {
                    result.push(' ');
                }
            }

            result.push(')');
        }
        _ => panic!("Unsupported LispCell type"),
    }
}
