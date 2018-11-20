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
        LispCell::Func(ref func) => {
            result.push_str(format!("#{}", &func.name).as_str())
        }
        LispCell::Quoted(ref quoted) => {
            let mut quoted_result = String::new();
            print_rec(quoted.clone(), &mut quoted_result);

            result.push_str(format!("'{}", quoted_result).as_str());
        }
        LispCell::Number(num) => result.push_str(num.to_string().as_str()),
        LispCell::Atom(ref atom) => result.push_str(atom.as_str()),
        LispCell::List(ref list) => {
            result.push('(');

            let list_vec = LispList::to_vec(list.clone());

            let n = list_vec.len();
            for i in 0..n {
                let cell = &list_vec[i];

                print_rec(cell.clone(), result);

                if i != n - 1 {
                    result.push(' ');
                }
            }

            result.push(')');
        }
        ref c @ _ => panic!("Unsupported LispCell type: {:?}", c),
    }
}
