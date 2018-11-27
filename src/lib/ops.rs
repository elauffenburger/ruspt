use std::cell::RefCell;
use std::rc::Rc;

use super::core::{self, log};
use super::{exec, Environment, LispCell, LispCellRef, LispFunc, LispFuncExecutor, LispFuncType, LispList};

pub fn add(_env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    Rc::new(RefCell::new(LispCell::Number(to_nums(args).sum())))
}

pub fn sub(_env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    let mut nums = to_nums(args);
    let first = nums.next().unwrap();

    LispCell::Number(nums.fold(first, |acc, val| acc - val)).to_ref()
}

pub fn mul(_env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    LispCell::Number(to_nums(args).product()).to_ref()
}

pub fn div(_env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    let mut nums = to_nums(args);
    let first = nums.next().unwrap();

    LispCell::Number(nums.fold(first, |acc, val| acc / val)).to_ref()
}

pub fn list(_env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    LispCell::new_list(args.clone())
}

pub fn def(env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    let mut iter = args.iter();

    let cell = iter.next().unwrap();
    match *cell.borrow() {
        LispCell::Atom(ref symbol) => {
            let value = exec(env.clone(), iter.next().unwrap().clone());

            log(|| println!("Defining symbol: {:?} with value: {:?}", symbol, value));

            env.borrow_mut().def(symbol.clone(), value);

            log(|| println!("Symbol {:?} defined", symbol));

            cell.clone()
        }
        _ => panic!("Unable to find symbol to define in call to def"),
    }
}

pub fn defn(env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [arg1, arg2, arg3] => match (&*arg1.borrow(), &*arg2.borrow(), arg3) {
            (LispCell::Atom(ref func_name), LispCell::List(ref func_args), func_body) => {
                log(|| println!("preparing to defn {}", func_name));

                let cloned_func_args = func_args.clone();

                let arg_names: Vec<String> = LispList::to_vec(cloned_func_args)
                    .iter()
                    .map(|arg| {
                        let unwrapped_arg = arg.clone();
                        let borrowed_arg = unwrapped_arg.borrow();

                        match *borrowed_arg {
                            LispCell::Atom(ref name) => name.clone(),
                            _ => panic!("Non-atom arg passed in func args list: {:?}", &func_args),
                        }
                    }).collect();

                let func_executor = Box::new(DefnFuncExecutorImpl {
                    name: func_name.clone(),
                    arg_names: arg_names,
                    func_body: func_body.clone(),
                    env: None,
                });

                let func =
                    LispCell::Func(LispFunc::new(func_name.clone(), LispFuncType::Normal, func_executor)).to_ref();

                env.borrow_mut().def(func_name.clone(), func.clone());

                func
            }
            _ => panic!("Invalid arg types passed to defn (expecting name, args, and body): {:?}", &args),
        },
        _ => panic!("Invalid number of args passed to defn: {:?}", &args),
    }
}

pub fn dew(env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    // Execute each arg in the vec and return the last expr result as the result
    args.iter().map(|arg| exec(env.clone(), arg.clone())).last().unwrap()
}

pub fn push(_env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [el, list_arg] => match *list_arg.borrow_mut() {
            LispCell::List(ref list) => {
                list.borrow_mut().push_back(el.clone());

                list_arg.clone()
            }
            ref l @ _ => panic!("Second arg passed to push was not a list: {:?}", l),
        },
        _ => panic!("Invalid arg num passed to push: {:?}", &args),
    }
}

pub fn car(_env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [list_arg] => match *list_arg.borrow() {
            LispCell::List(ref list) => match list.borrow().get_value() {
                Some(value) => value,
                _ => core::lisp_null(),
            },
            ref l @ _ => panic!("Arg passed to push was not a list: {:?}", l),
        },
        _ => panic!("Invalid arg num passed to push: {:?}", &args),
    }
}

pub fn cdr(_env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [list] => match *list.borrow() {
            LispCell::List(ref list) => {
                let (_, rest) = LispList::split(list.clone());

                match rest {
                    Some(rest) => LispCell::List(rest).to_ref(),
                    None => core::lisp_null(),
                }
            }
            ref l @ _ => panic!("Arg passed to push was not a list: {:?}", l),
        },
        _ => panic!("Invalid arg num passed to push: {:?}", &args),
    }
}

pub fn iff(env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [pred, true_case, false_case] => {
            let pred_result = exec(env.clone(), pred.clone());
            let borrowed_pred_result = pred_result.borrow();

            match *borrowed_pred_result {
                LispCell::Bool(true) => exec(env.clone(), true_case.clone()),
                LispCell::Bool(false) => exec(env.clone(), false_case.clone()),
                ref r @ _ => panic!("Invalid result returned by if predicate: {:?}", r),
            }
        }
        _ => panic!("Invalid arg num passed to if: {:?}", &args),
    }
}

pub fn eq(_env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [left, right] => {
            let is_eq = left == right;

            Rc::new(RefCell::new(LispCell::Bool(is_eq)))
        }
        _ => panic!("Invalid arg num passed to if: {:?}", &args),
    }
}

pub fn lambda(env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [lambda_args, lambda_body] => match (&*lambda_args.borrow(), &*lambda_body.borrow()) {
            (LispCell::List(ref lambda_args), LispCell::List(_)) => {
                log(|| println!("preparing to lambda {:?} {:?}", lambda_args, lambda_body));

                let cloned_func_args = lambda_args.clone();

                let arg_names: Vec<String> = LispList::to_vec(cloned_func_args)
                    .iter()
                    .map(|arg| {
                        let unwrapped_arg = arg.clone();
                        let borrowed_arg = unwrapped_arg.borrow();

                        match *borrowed_arg {
                            LispCell::Atom(ref name) => name.clone(),
                            _ => panic!("Non-atom arg passed in func args list: {:?}", &lambda_args),
                        }
                    }).collect();

                let func_name = String::from("lambda");

                let func_executor = Box::new(DefnFuncExecutorImpl {
                    name: func_name.clone(),
                    arg_names: arg_names,
                    func_body: lambda_body.clone(),
                    env: Some(Rc::new(RefCell::new(Environment::new_child(env)))),
                });

                let func = LispCell::Func(LispFunc::new(func_name, LispFuncType::Normal, func_executor)).to_ref();

                func
            }
            _ => panic!("Invalid args passed to lambda: {:?}", &args),
        },
        _ => panic!("Invalid arg num passed to lambda: {:?}", &args),
    }
}

fn to_nums<'a>(args: &'a Vec<LispCellRef>) -> Box<Iterator<Item = f32> + 'a> {
    let map = args.iter().map(|arg| match *arg.borrow() {
        LispCell::Number(num) => num,
        ref c @ _ => panic!("Non-number cell handed to numeric operator: {:?}", c),
    });

    Box::new(map)
}

struct DefnFuncExecutorImpl {
    name: String,
    func_body: LispCellRef,
    arg_names: Vec<String>,
    env: Option<Rc<RefCell<Environment>>>,
}

impl LispFuncExecutor for DefnFuncExecutorImpl {
    fn exec(&self, env: Rc<RefCell<Environment>>, args: &Vec<LispCellRef>) -> LispCellRef {
        log(|| println!("exec'ing {}", &self.name));

        let env = match self.env {
            Some(ref env) => env.clone(),
            None => env,
        };

        {
            let n = args.len();
            let expected_n = self.arg_names.len();

            if n != expected_n {
                panic!("number of args provided ({}) does not equal expected num ({})", n, expected_n)
            }

            let mut i = 0;
            self.arg_names.iter().for_each(|name| {
                env.borrow_mut().def(name.clone(), args[i].clone());
                i += 1;
            });
        }

        exec(env, self.func_body.clone())
    }
}
