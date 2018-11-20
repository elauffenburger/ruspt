mod lisp_cell;
mod lisp_list;
mod lisp_func;
mod env;

pub use self::lisp_cell::*;
pub use self::lisp_list::*;
pub use self::lisp_func::*;
pub use self::env::*;

use std::cell::RefCell;
use std::fmt::{self, Debug};
use std::rc::Rc;

use super::ops;

#[derive(Debug, PartialEq)]
pub struct LispProgram {
    pub text: String,
    pub entry: Option<Rc<RefCell<LispCell>>>,
}

pub fn lisp_null() -> LispCellRef {
    LispCell::new_list(vec![])
}

pub fn log<F>(log_fn: F)
where
    F: FnOnce(),
{
    if cfg!(feature = "core_debug") {
        log_fn();
    }
}