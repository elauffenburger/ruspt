use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::rc::Rc;

use super::ops;
use super::util;

pub type LispCellRef = Rc<RefCell<LispCell>>;
type LispListRef = Rc<RefCell<LispList>>;

#[derive(Debug, Clone, PartialEq)]
pub enum LispCell {
    Atom(String),
    Number(f32),
    Bool(bool),
    Str(String),
    Quoted(LispCellRef),
    Func(LispFunc),
    List(Rc<RefCell<LispList>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LispList {
    value: LispCellRef,
    next: Option<Rc<RefCell<LispList>>>,
}

impl LispList {
    pub fn new() -> Option<LispList> {
        Self::from_vec(vec![])
    }

    pub fn from_value(value: LispCellRef) -> LispList {
        LispList {
            value: value,
            next: None,
        }
    }

    pub fn from_vec(vec: Vec<LispCellRef>) -> Option<LispList> {
        if vec.len() == 0 {
            return None;
        }

        let mut head = None;

        {
            let mut current: Option<Rc<RefCell<LispList>>> = None;

            vec.iter().for_each(|cell| {
                let next = LispList::from_value(cell.clone()).to_ref();

                match head {
                    None => {
                        head = Some(next.clone());
                        current = Some(next);
                    }
                    _ => {
                        let borrowed_current = current.clone().expect("current should be init'd after init'ing head");
                        borrowed_current.borrow_mut().next = Some(next.clone());
                        current = Some(next);
                    }
                }
            });
        }

        Some(Rc::try_unwrap(head.unwrap()).unwrap().into_inner())
    }

    pub fn split(list: LispListRef) -> (LispListRef, Option<LispListRef>) {
        let head = list.clone();
        let rest = list.borrow().next.clone();

        (head, rest)
    }

    pub fn to_ref(self) -> LispListRef {
        Rc::new(RefCell::new(self))
    }

    pub fn get_value(&self) -> LispCellRef {
        self.value.clone()
    }

    pub fn to_vec(list: LispListRef) -> Vec<LispCellRef> {
        let mut results = vec![];
        let mut current = Some(list);

        loop {
            match current {
                None => break,
                Some(node) => {
                    let borrowed_node = node.borrow();

                    results.push(borrowed_node.get_value());
                    current = borrowed_node.next.clone();
                }
            }
        }

        results
    }

    pub fn push_back(&mut self, node: LispCellRef) -> LispListRef {
        let orig_next = self.next.clone();
        let new_node = LispList::from_value(node).to_ref();

        self.next = Some(new_node.clone());

        match orig_next {
            None => {}
            Some(_) => {
                new_node.borrow_mut().next = orig_next;
            }
        }

        new_node
    }
}

impl LispCell {
    pub fn new_list(list: Vec<LispCellRef>) -> LispCellRef {
        LispCell::List(LispList::from_vec(list).unwrap().to_ref()).to_ref()
    }

    pub fn to_ref(self) -> LispCellRef {
        Rc::new(RefCell::new(self))
    }
}

pub struct LispFunc {
    pub name: String,
    pub func_type: LispFuncType,
    pub func: Rc<LispFn>,
}

impl LispFunc {
    pub fn new(name: String, func_type: LispFuncType, func: Rc<LispFn>) -> LispFunc {
        LispFunc { name: name, func_type: func_type, func: func }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LispFuncType {
    Macro,
    SpecialForm,
    Normal,
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
    pub entry: Option<Rc<RefCell<LispCell>>>,
}

pub type LispFn = Fn(&mut Environment, &Vec<LispCellRef>) -> LispCellRef;

pub struct Environment {
    pub symbols: HashMap<String, Rc<RefCell<LispCell>>>,
}

impl Environment {
    pub fn def<'a>(&mut self, symbol: String, cell: Rc<RefCell<LispCell>>) {
        log(|| println!("symbols: {:?}", &self.symbols));

        self.symbols.insert(symbol, cell);
    }

    pub fn find_sym(&self, name: &String) -> Option<Rc<RefCell<LispCell>>> {
        log(|| println!("looking up symbol {}", name));

        match self.symbols.get(name) {
            Some(sym) => Some(sym.clone()),
            None => None,
        }
    }
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Environment { symbols: self.symbols.clone() }
    }
}

pub fn new_env() -> Environment {
    Environment {
        symbols: make_builtin_symbols(),
    }
}

fn make_builtin_symbols() -> HashMap<String, Rc<RefCell<LispCell>>> {
    let mut map: HashMap<String, Rc<RefCell<LispCell>>> = HashMap::new();

    add_op("+", LispFuncType::Normal, Rc::new(ops::add), &mut map);
    add_op("-", LispFuncType::Normal, Rc::new(ops::sub), &mut map);
    add_op("*", LispFuncType::Normal, Rc::new(ops::mul), &mut map);
    add_op("/", LispFuncType::Normal, Rc::new(ops::div), &mut map);
    add_op("list", LispFuncType::Normal, Rc::new(ops::list), &mut map);
    add_op("def", LispFuncType::SpecialForm, Rc::new(ops::def), &mut map);
    add_op("defn", LispFuncType::SpecialForm, Rc::new(ops::defn), &mut map);
    add_op("do", LispFuncType::SpecialForm, Rc::new(ops::dew), &mut map);
    add_op("push", LispFuncType::Normal, Rc::new(ops::push), &mut map);
    add_op("car", LispFuncType::Normal, Rc::new(ops::car), &mut map);
    add_op("cdr", LispFuncType::Normal, Rc::new(ops::cdr), &mut map);
    add_op("if", LispFuncType::SpecialForm, Rc::new(ops::iff), &mut map);
    add_op("eq", LispFuncType::Normal, Rc::new(ops::eq), &mut map);

    map
}

fn add_op(name: &'static str, func_type: LispFuncType, op: Rc<LispFn>, map: &mut HashMap<String, Rc<RefCell<LispCell>>>) {
    map.insert(
        name.to_string(),
        Rc::new(RefCell::new(LispCell::Func(LispFunc {
            name: name.to_string(),
            func_type: func_type,
            func: op,
        }))),
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

#[cfg(test)]
mod test {
    use super::util;
    use super::*;

    #[test]
    fn create_list_from_vec() {
        let list_contents = vec![util::make_num(1f32)];
        let list = LispList::from_vec(list_contents.clone()).unwrap().to_ref();

        assert_eq!(LispList::to_vec(list), list_contents);
    }
}
