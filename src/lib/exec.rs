use super::core::*;
use super::util;

use std::cell::RefCell;
use std::rc::Rc;

pub fn exec_prog(mut env: Environment, program: LispProgram) -> LispCellRef {
    match program.entry {
        Some(e) => exec_rec(&mut env, e),
        _ => panic!("No entry found for program!"),
    }
}

pub fn exec(env: &mut Environment, cell: LispCellRef) -> LispCellRef {
    exec_rec(env, cell)
}

fn exec_rec(env: &mut Environment, cell: LispCellRef) -> LispCellRef {
    match *cell.borrow() {
        LispCell::Atom(ref symbol) => {
            let maybe_sym = env.find_sym(symbol);

            match maybe_sym {
                Some(sym) => sym.clone(),
                None => panic!("No symbol found with name {}", symbol),
            }
        }
        LispCell::Quoted(ref quoted) => {
            log(|| println!("Unquoting {:?}", quoted));

            quoted.clone()
        }
        LispCell::Number(_) => cell.clone(),
        LispCell::List {
            ref contents,
        } => {
            let mut borrowed_contents = contents.borrow_mut();

            unsafe {
                let (x, xs) = util::split_at_head(&mut borrowed_contents);

                let function = exec_rec(env, x.unwrap().clone());
                let args: Vec<LispCellRef> = xs.iter().map(|cell| cell.clone()).collect();

                call_fn(env, function, &args)
            }
        }
        ref t @ _ => panic!("LispCell type {:?} not implemented in exec!", t),
    }
}

fn call_fn(env: &mut Environment, function_cell: LispCellRef, args: &[LispCellRef]) -> LispCellRef {
    match *function_cell.borrow() {
        LispCell::Func(ref function) => {
            let args = match function.func_type {
                LispFuncType::Macro | LispFuncType::SpecialForm => args.to_vec(),
                _ => args.clone().iter().map(|cell| exec_rec(env, cell.clone())).collect(),
            };

            (function.func)(env, &args)
        }
        ref t @ _ => panic!("LispCell type {:?} not a Func!", t),
    }
}
