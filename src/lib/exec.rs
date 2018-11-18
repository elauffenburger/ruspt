use super::core::*;

use std::rc::Rc;
use std::cell::RefCell;

pub fn exec_prog(mut env: Environment, program: LispProgram) -> Rc<RefCell<LispCell>> {
    match program.entry {
        Some(e) => exec_rec(&mut env, e),
        _ => panic!("No entry found for program!"),
    }
}

pub fn exec(env: &mut Environment, cell: Rc<RefCell<LispCell>>) -> Rc<RefCell<LispCell>> {
    exec_rec(env, cell)
}

fn exec_rec(env: &mut Environment, cell: Rc<RefCell<LispCell>>) -> Rc<RefCell<LispCell>> {
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
        },
        LispCell::Number(_) => cell.clone(),
        LispCell::List {
            ref contents,
        } => {
            let (x, xs) = contents.split_at(1);

            let function = exec_rec(env, x.get(0).unwrap().clone());

            call_fn(env, function, xs)
        }
        ref t @ _ => panic!("LispCell type {:?} not implemented in exec!", t),
    }
}

fn call_fn(env: &mut Environment, function_cell: Rc<RefCell<LispCell>>, args: &[Rc<RefCell<LispCell>>]) -> Rc<RefCell<LispCell>> {
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
