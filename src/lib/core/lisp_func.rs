use super::*;

pub struct LispFunc {
    pub name: String,
    pub func_type: LispFuncType,
    pub func_executor: Rc<Box<LispFuncExecutor>>,
}

impl LispFunc {
    pub fn new(name: String, func_type: LispFuncType, func_executor: Box<LispFuncExecutor>) -> LispFunc {
        LispFunc {
            name: name,
            func_type: func_type,
            func_executor: Rc::new(func_executor),
        }
    }
}

pub trait LispFuncExecutor {
    fn exec(&self, env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef;
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
            func_executor: self.func_executor.clone(),
        }
    }
}