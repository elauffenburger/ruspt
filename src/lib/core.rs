use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::rc::Rc;

use super::ops;

#[derive(Debug, Clone, PartialEq)]
pub enum LispCell {
    Atom(String),
    Number(f32),
    Str(String),
    Quoted(Rc<LispCell>),
    Func(LispFunc),
    List {
        contents: Vec<Rc<LispCell>>,
    },
}

pub struct LispFunc {
    pub name: String,
    pub func_type: LispFuncType,
    pub func: Rc<LispFn>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LispFuncType {
    Macro,
    SpecialForm,
    Normal
}

impl Debug for LispFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LispFunc {{ name: {}, func: ... }}", self.name)
    }
}

impl PartialEq for LispFunc {
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name
    }
}

impl Clone for LispFunc {
    fn clone(&self) -> Self {
        LispFunc {
            name: self.name.clone(),
            func_type: self.func_type.clone(),
            func: self.func.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LispProgram {
    pub text: String,
    pub entry: Option<Rc<LispCell>>,
}

pub type LispFn = Fn(&mut Environment, &Vec<Rc<LispCell>>) -> Rc<LispCell>;

pub struct Environment {
    pub symbols: HashMap<String, Rc<LispCell>>,
}

impl Environment {
    pub fn def<'a>(&mut self, symbol: String, cell: Rc<LispCell>) {
        log(|| println!("symbols: {:?}", &self.symbols));

        self.symbols.insert(symbol, cell);
    }

    pub fn find_sym(&self, name: &String) -> Option<Rc<LispCell>> {
        log(|| println!("looking up symbol {}", name));

        match self.symbols.get(name) {
            Some(sym) => Some(sym.clone()),
            None => None,
        }
    }
}

pub fn new_env() -> Environment {
    Environment {
        symbols: make_builtin_symbols(),
    }
}

fn make_builtin_symbols() -> HashMap<String, Rc<LispCell>> {
    let mut map: HashMap<String, Rc<LispCell>> = HashMap::new();

    add_op("+", LispFuncType::Normal, Rc::new(ops::add), &mut map);
    add_op("-", LispFuncType::Normal, Rc::new(ops::sub), &mut map);
    add_op("*", LispFuncType::Normal, Rc::new(ops::mul), &mut map);
    add_op("/", LispFuncType::Normal, Rc::new(ops::div), &mut map);
    add_op("def", LispFuncType::SpecialForm, Rc::new(ops::def), &mut map);
    add_op("do", LispFuncType::SpecialForm, Rc::new(ops::do_fn), &mut map);

    map
}

fn add_op(name: &'static str, func_type: LispFuncType, op: Rc<LispFn>, map: &mut HashMap<String, Rc<LispCell>>) {
    map.insert(
        name.to_string(),
        Rc::new(LispCell::Func(LispFunc {
            name: name.to_string(),
            func_type: func_type,
            func: op,
        })),
    );
}

pub fn log<F>(logFn: F)
where
    F: FnOnce(),
{
    if cfg!(feature = "core_debug") {
        logFn();
    }
}
