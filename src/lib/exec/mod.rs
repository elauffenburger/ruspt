mod ops;

use super::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn exec_prog(mut env: Environment, program: LispProgram) -> LispCell {
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

            call_fn(env, function, &args)
        }
        t @ _ => panic!("LispCell type {:?} not implemented!", t),
    }
}

fn call_fn(env: &mut Environment, function_cell: &LispCell, args: &Vec<LispCell>) -> LispCell {
    let fn_clone = env.functions.clone();
    match function_cell {
        LispCell::Atom(atom) => match fn_clone.borrow().get(atom.as_str()) {
            Some(function) => function(env, args),
            None => panic!("Function {:?} not found!", atom),
        },
        t @ _ => panic!("LispCell type {:?} not implemented!", t),
    }
}

pub type LispFn = Fn(&mut Environment, &Vec<LispCell>) -> LispCell;

pub struct Environment {
    functions: Rc<RefCell<HashMap<String, Box<LispFn>>>>,
    symbols: Rc<RefCell<HashMap<String, Box<LispCell>>>>,
}

impl Environment {
    pub fn def<'a>(&mut self, symbol: String, cell: LispCell) {
        self.symbols.borrow_mut().insert(symbol, Box::new(cell)).unwrap();
    }
}

pub fn new_env() -> Environment {
    Environment {
        functions: Rc::new(RefCell::new(make_builtin_functions())),
        symbols: Rc::new(RefCell::new(make_builtin_symbols())),
    }
}

fn make_builtin_symbols() -> HashMap<String, Box<LispCell>> {
    HashMap::new()
}

fn make_builtin_functions() -> HashMap<String, Box<LispFn>> {
    let mut map: HashMap<String, Box<LispFn>> = HashMap::new();

    map.insert("+".to_string(), Box::new(ops::add));
    map.insert("-".to_string(), Box::new(ops::sub));
    map.insert("*".to_string(), Box::new(ops::mul));
    map.insert("/".to_string(), Box::new(ops::div));
    map.insert("def".to_string(), Box::new(ops::def));

    map
}
