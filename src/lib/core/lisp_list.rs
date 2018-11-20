use super::*;

use std::rc::Rc;
use std::cell::RefCell;

type LispListRef = Rc<RefCell<LispList>>;

#[derive(Debug, Clone, PartialEq)]
pub struct LispList {
    value: Option<LispCellRef>,
    next: Option<LispListRef>,
}

impl LispList {
    pub fn new() -> LispList {
        Self::from_vec(vec![])
    }

    pub fn from_value(value: LispCellRef) -> LispList {
        LispList {
            value: Some(value),
            next: None,
        }
    }

    pub fn from_vec(vec: Vec<LispCellRef>) -> LispList {
        if vec.len() == 0 {
            return LispList {
                value: None,
                next: None,
            };
        }

        let mut head = None;

        {
            let mut current: Option<LispListRef> = None;

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

        Rc::try_unwrap(head.unwrap()).unwrap().into_inner()
    }

    pub fn split(list: LispListRef) -> (LispListRef, Option<LispListRef>) {
        let head = list.clone();
        let rest = list.borrow().next.clone();

        (head, rest)
    }

    pub fn to_ref(self) -> LispListRef {
        Rc::new(RefCell::new(self))
    }

    pub fn get_value(&self) -> Option<LispCellRef> {
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

                    match borrowed_node.get_value() {
                        Some(value) => results.push(value.clone()),
                        None => {}
                    };

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

#[cfg(test)]
mod test {
    use util;
    use super::*;

    #[test]
    fn create_list_from_vec() {
        let list_contents = vec![util::make_num(1f32)];
        let list = LispList::from_vec(list_contents.clone()).to_ref();

        assert_eq!(LispList::to_vec(list), list_contents);
    }
}