use super::core::*;
use super::util;

use std::cell::RefCell;
use std::rc::Rc;

pub fn exec_prog(mut env: &mut Environment, program: LispProgram) -> LispCellRef {
    match program.entry {
        Some(e) => exec_rec(env, e),
        _ => panic!("No entry found for program!"),
    }
}

pub fn exec(env: &mut Environment, cell: LispCellRef) -> LispCellRef {
    exec_rec(env, cell)
}

pub fn exec_ref(env: &mut Environment, cell_ref: &LispCellRef) -> LispCellRef {
    exec_rec(env, cell_ref.clone())
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
        LispCell::List(ref list) => {
            let (x, xs) = LispList::split(list.clone());

            let function = exec_rec(env, x.borrow().get_value().expect("There should be a value in the head of the args list to exec"));
            let args = match xs {
                Some(cells) => LispList::to_vec(cells).iter().map(|cell| cell.clone()).collect::<Vec<LispCellRef>>(),
                None => vec![]
            } ;

            call_fn(env, function, &args)
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

            function.func_executor.exec(env, &args)
        }
        ref t @ _ => panic!("LispCell type {:?} not a Func!", t),
    }
}
