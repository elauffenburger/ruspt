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
    Quoted(Box<LispCell>),
    Func(LispFunc),
    List {
        contents: Vec<LispCell>,
    },
}

pub struct LispFunc {
    pub name: String,
    pub func: Rc<Box<LispFn>>,
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
        LispFunc{ name: self.name, func: self.func.clone() }
    }
}

#[derive(Debug, PartialEq)]
pub struct LispProgram {
    pub text: String,
    pub entry: Option<Box<LispCell>>,
}

pub type LispFn = Fn(&mut Environment, &Vec<LispCell>) -> Box<LispCell>;

pub struct Environment {
    pub symbols: Rc<RefCell<HashMap<String, Box<LispCell>>>>,
}

impl Environment {
    pub fn def<'a>(&mut self, symbol: String, cell: LispCell) {
        self.symbols.borrow_mut().insert(symbol, Box::new(cell)).unwrap();
    }

    pub fn find_sym(&self, name: &String) -> Option<&Box<LispCell>> {
        self.symbols.borrow().get(name)
    }
}

pub fn new_env() -> Environment {
    Environment {
        symbols: Rc::new(RefCell::new(make_builtin_symbols())),
    }
}

fn make_builtin_symbols() -> HashMap<String, Box<LispCell>> {
    let mut map: HashMap<String, Box<LispCell>> = HashMap::new();

    add_op("+", Box::new(ops::add), &mut map);
    add_op("-", Box::new(ops::sub), &mut map);
    add_op("*", Box::new(ops::mul), &mut map);
    add_op("/", Box::new(ops::div), &mut map);
    add_op("def", Box::new(ops::def), &mut map);
    add_op("do", Box::new(ops::do_fn), &mut map);

    map
}


fn add_op(name: &'static str, op: Box<LispFn>, map: &mut HashMap<String, Box<LispCell>>) {
    let name = name.to_string();
    map.insert(name, Box::new(LispCell::Func(LispFunc{ name: name, func: Rc::new(op) })));
}
