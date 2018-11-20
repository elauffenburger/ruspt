use super::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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

    pub fn new() -> Environment {
        Environment {
            symbols: Self::make_builtin_symbols(),
        }
    }

    fn make_builtin_symbols() -> HashMap<String, Rc<RefCell<LispCell>>> {
        let mut map: HashMap<String, Rc<RefCell<LispCell>>> = HashMap::new();

        Self::add_op("+", LispFuncType::Normal, Rc::new(ops::add), &mut map);
        Self::add_op("-", LispFuncType::Normal, Rc::new(ops::sub), &mut map);
        Self::add_op("*", LispFuncType::Normal, Rc::new(ops::mul), &mut map);
        Self::add_op("/", LispFuncType::Normal, Rc::new(ops::div), &mut map);
        Self::add_op("list", LispFuncType::Normal, Rc::new(ops::list), &mut map);
        Self::add_op("def", LispFuncType::SpecialForm, Rc::new(ops::def), &mut map);
        Self::add_op("defn", LispFuncType::SpecialForm, Rc::new(ops::defn), &mut map);
        Self::add_op("do", LispFuncType::SpecialForm, Rc::new(ops::dew), &mut map);
        Self::add_op("push", LispFuncType::Normal, Rc::new(ops::push), &mut map);
        Self::add_op("car", LispFuncType::Normal, Rc::new(ops::car), &mut map);
        Self::add_op("cdr", LispFuncType::Normal, Rc::new(ops::cdr), &mut map);
        Self::add_op("if", LispFuncType::SpecialForm, Rc::new(ops::iff), &mut map);
        Self::add_op("eq", LispFuncType::Normal, Rc::new(ops::eq), &mut map);

        map
    }

    fn add_op(name: &'static str, func_type: LispFuncType, op: Rc<LispFn>, map: &mut HashMap<String, Rc<RefCell<LispCell>>>) {
        map.insert(
            name.to_string(),
            Rc::new(RefCell::new(LispCell::Func(LispFunc {
                name: name.to_string(),
                func_type: func_type,
                func_executor: Rc::new(Box::new(FnLispFuncExecutor {
                    op: op,
                })),
            }))),
        );
    }
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Environment {
            symbols: self.symbols.clone(),
        }
    }
}

type LispFn = Fn(&mut Environment, &Vec<LispCellRef>) -> LispCellRef;

struct FnLispFuncExecutor {
    op: Rc<LispFn>,
}

impl LispFuncExecutor for FnLispFuncExecutor {
    fn exec(&self, env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
        (self.op)(env, args)
    }
}
