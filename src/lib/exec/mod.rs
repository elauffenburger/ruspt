mod ops;

use std::rc::Rc;
use super::*;
use std::collections::HashMap;

pub fn exec_prog(env: Environment, program: LispProgram) -> LispCell {
    match program.entry {
        Some(e) => exec_rec(&mut env, &*e),
        _ => panic!("No entry found for program!"),
    }
}

pub fn exec(env: &mut Environment, cell: &LispCell) -> LispCell {
    exec_rec(env, cell)
}

fn exec_rec(env: &mut Environment, cell: &LispCell) -> LispCell {
    match &cell {
        LispCell::Atom(_) | LispCell::Number(_) => cell.clone(),
        LispCell::List {
            ref contents,
        } => {
            let (x, xs) = contents.split_at(1);

            let function = x.get(0).unwrap();
            let args = xs.clone().iter().map(|cell| exec_rec(env, cell)).collect();

            call_fn(&mut env, function, &args)
        }
        t @ _ => panic!("LispCell type {:?} not implemented!", t),
    }
}

fn call_fn(env: &mut Environment, function_cell: &LispCell, args: &Vec<LispCell>) -> LispCell {
    match function_cell {
        LispCell::Atom(atom) => {
            let function = {
                let maybe_fn = env.functions.get(atom.as_str());

                match maybe_fn {
                    Some(function) => function.clone(),
                    None => panic!("Function {:?} not found!", atom),
                }
            };

            function(env, args)
        }
        t @ _ => panic!("LispCell type {:?} not implemented!", t),
    }
}

pub type LispFn = Fn(&mut Environment, &Vec<LispCell>) -> LispCell;

pub struct Environment {
    functions: HashMap<String, Rc<LispFn>>,
    symbols: HashMap<String, Box<LispCell>>,
}

impl Environment {
    pub fn def<'a>(&mut self, symbol: String, cell: LispCell) {
        self.symbols.insert(symbol, Box::new(cell)).unwrap();
    }
}

pub fn new_env() -> Environment {
    Environment {
        functions: make_builtin_functions(),
        symbols: make_builtin_symbols(),
    }
}

fn make_builtin_symbols() -> HashMap<String, Box<LispCell>> {
    HashMap::new()
}

fn make_builtin_functions() -> HashMap<String, Rc<LispFn>> {
    let mut map = HashMap::new();

    map.insert("+".to_string(), Rc::new(ops::add));
    map.insert("-".to_string(), Rc::new(ops::sub));
    map.insert("*".to_string(), Rc::new(ops::mul));
    map.insert("/".to_string(), Rc::new(ops::div));
    map.insert("def".to_string(), Rc::new(ops::def));

    map
}
