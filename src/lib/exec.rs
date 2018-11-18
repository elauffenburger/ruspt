use super::core::*;

use std::rc::Rc;

pub fn exec_prog(mut env: Environment, program: LispProgram) -> Rc<LispCell> {
    match program.entry {
        Some(e) => exec_rec(&mut env, &*e),
        _ => panic!("No entry found for program!"),
    }
}

pub fn exec(env: &mut Environment, cell: &LispCell) -> Rc<LispCell> {
    exec_rec(env, cell)
}

fn exec_rec(env: &mut Environment, cell: &LispCell) -> Rc<LispCell> {
    match &cell {
        LispCell::Atom(symbol) => {
            let maybe_sym = env.find_sym(symbol);

            match maybe_sym {
                Some(sym) => sym.clone(),
                None => panic!("No symbol found with name {}", symbol),
            }
        }
        LispCell::Number(_) => Rc::new(cell.clone()),
        LispCell::List {
            ref contents,
        } => {
            let (x, xs) = contents.split_at(1);

            let function = exec_rec(env, x.get(0).unwrap());

            call_fn(env, function, xs)
        }
        t @ _ => panic!("LispCell type {:?} not implemented in exec!", t),
    }
}

fn call_fn(env: &mut Environment, function_cell: Rc<LispCell>, args: &[Rc<LispCell>]) -> Rc<LispCell> {
    match *function_cell.clone() {
        LispCell::Func(ref function) => {
            let args = match function.func_type {
                LispFuncType::Macro | LispFuncType::SpecialForm => args.to_vec(),
                _ => args.clone().iter().map(|cell| exec_rec(env, cell)).collect(),
            };

            (function.func)(env, &args)
        }
        ref t @ _ => panic!("LispCell type {:?} not a Func!", t),
    }
}
