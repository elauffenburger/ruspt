use super::*;

use std::rc::Rc;
use std::cell::RefCell;

pub type LispCellRef = Rc<RefCell<LispCell>>;

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

impl LispCell {
    pub fn new_list(list: Vec<LispCellRef>) -> LispCellRef {
        LispCell::List(LispList::from_vec(list).to_ref()).to_ref()
    }

    pub fn to_ref(self) -> LispCellRef {
        Rc::new(RefCell::new(self))
    }
}