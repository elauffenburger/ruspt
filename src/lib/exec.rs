use super::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn exec_prog(mut env: Environment, program: LispProgram) -> Box<LispCell> {
    match program.entry {
        Some(e) => exec_rec(&mut env, &*e),
        _ => panic!("No entry found for program!"),
    }
}

pub fn exec(env: &mut Environment, cell: &LispCell) -> Box<LispCell> {
    exec_rec(env, cell)
}

fn exec_rec(env: &mut Environment, cell: &LispCell) -> Box<LispCell> {
    match &cell {
        LispCell::Atom(symbol) => {
            let maybe_sym = env.find_sym(symbol);

            match maybe_sym {
                Some(sym) => sym,
                None => panic!("No symbol found with name {}", symbol)
            }
        }
        LispCell::Number(_) => cell.clone(),
        LispCell::List {
            ref contents,
        } => {
            let (x, xs) = contents.split_at(1);

            let function = x.get(0).unwrap();
            let args = xs.clone().iter().map(|cell| exec_rec(env, cell)).collect();

            call_fn(env, function, &args)
        }
        t @ _ => panic!("LispCell type {:?} not implemented!", t),
    }
}

fn call_fn(env: &mut Environment, function_cell: &LispCell, args: &Vec<LispCell>) -> Box<LispCell> {
    match function_cell {
        LispCell::Func(function) => (function.func)(env, args),
        t @ _ => panic!("LispCell type {:?} not implemented!", t),
    }
}