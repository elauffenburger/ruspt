use std::cell::RefCell;
use std::collections::linked_list::LinkedList;
use std::rc::Rc;

use super::{LispCell, LispCellRef};

pub fn split_at_head<T>(list: &mut LinkedList<Rc<T>>) -> (Option<Rc<T>>, LinkedList<Rc<T>>) {
    let head = match list.front() {
        Some(head) => Some(head.clone()),
        None => None,
    };

    (head, list.split_off(1))
}

pub fn make_num(num: f32) -> LispCellRef {
    Rc::new(RefCell::new(LispCell::Number(num)))
}

pub fn make_atom(name: &'static str) -> LispCellRef {
    Rc::new(RefCell::new(LispCell::Atom(name.to_string())))
}

pub fn make_list(list: Vec<LispCellRef>) -> LispCellRef {
    LispCell::new_list(list)
}

pub fn make_quoted(cell: LispCellRef) -> LispCellRef {
    Rc::new(RefCell::new(LispCell::Quoted(cell)))
}

pub fn make_bool(val: bool) -> LispCellRef {
    Rc::new(RefCell::new(LispCell::Bool(val)))
}
