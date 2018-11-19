use std::collections::linked_list::LinkedList;
use std::rc::Rc;

pub fn split_at_head<T>(list: &mut LinkedList<Rc<T>>) -> (Option<Rc<T>>, LinkedList<Rc<T>>) {
    let head = match list.front() {
        Some(head) => Some(head.clone()),
        None => None
    };

    (head, list.split_off(1))
}
