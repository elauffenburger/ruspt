use std::cell::RefCell;
use std::rc::Rc;

use super::core::log;
use super::{exec, Environment, LispCell, LispCellRef, LispFunc, LispFuncType, LispList};

pub fn add(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    Rc::new(RefCell::new(LispCell::Number(to_nums(args).sum())))
}

pub fn sub(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    let mut nums = to_nums(args);
    let first = nums.next().unwrap();

    LispCell::Number(nums.fold(first, |acc, val| acc - val)).to_ref()
}

pub fn mul(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    LispCell::Number(to_nums(args).product()).to_ref()
}

pub fn div(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    let mut nums = to_nums(args);
    let first = nums.next().unwrap();

    LispCell::Number(nums.fold(first, |acc, val| acc / val)).to_ref()
}

pub fn list(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    LispCell::new_list(args.clone())
}

pub fn def(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    let mut iter = args.iter();

    let cell = iter.next().unwrap();
    match *cell.borrow() {
        LispCell::Atom(ref symbol) => {
            let value = exec(env, iter.next().unwrap().clone());

            log(|| println!("Defining symbol: {:?} with value: {:?}", symbol, value));

            env.def(symbol.clone(), value);

            log(|| println!("Symbol {:?} defined", symbol));

            cell.clone()
        }
        _ => panic!("Unable to find symbol to define in call to def"),
    }
}

pub fn defn(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [arg1, arg2, arg3] => {
            match (&*arg1.borrow(), &*arg2.borrow(), arg3) {
                (LispCell::Atom(ref func_name), LispCell::List(ref func_args), ref func_body) => {
                    let cloned_func_args = func_args.clone();

                    let arg_names: Vec<String> = LispList::to_vec(cloned_func_args)
                        .iter()
                        .map(|arg| match *arg.borrow() {
                            LispCell::Atom(ref name) => name.clone(),
                            _ => panic!("Non-atom arg passed in func args list: {:?}", &func_args),
                        }).collect();

                    let func_impl = move |env: &mut Environment, args: &Vec<LispCellRef>| {
                        // {
                        //     let mut i = 0;
                        //     arg_names.iter().for_each(|name| {
                        //         env.def(name.clone(), args[i]);
                        //         i += 1;
                        //     });
                        // }

                        // let cloned_func_body = (&func_body).clone();

                        // exec(env, *cloned_func_body)

                        panic!("func_impl not implemented!")
                    };

                    let func = LispCell::Func(LispFunc::new(func_name.clone(), LispFuncType::Normal, Rc::new(func_impl))).to_ref();

                    env.def(func_name.clone(), func.clone());

                    func
                }
                _ => panic!("Invalid arg types passed to defn (expecting name, args, and body): {:?}", &args),
            }
        }
        _ => panic!("Invalid number of args passed to defn: {:?}", &args),
    }
}

pub fn dew(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    // Execute each arg in the vec and return the last expr result as the result
    args.iter().map(|arg| exec(env, arg.clone())).last().unwrap()
}

pub fn push(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
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

pub fn car(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [list_arg] => match *list_arg.borrow() {
            LispCell::List(ref list) => list.borrow().get_value(),
            ref l @ _ => panic!("Arg passed to push was not a list: {:?}", l),
        },
        _ => panic!("Invalid arg num passed to push: {:?}", &args),
    }
}

pub fn cdr(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [list] => match *list.borrow() {
            LispCell::List(ref list) => {
                let (_, rest) = LispList::split(list.clone());

                match rest {
                    Some(rest) => LispCell::List(rest).to_ref(),
                    None => lisp_null(),
                }
            }
            ref l @ _ => panic!("Arg passed to push was not a list: {:?}", l),
        },
        _ => panic!("Invalid arg num passed to push: {:?}", &args),
    }
}

pub fn iff(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [pred, true_case, false_case] => {
            let pred_result = exec(env, pred.clone());
            let borrowed_pred_result = pred_result.borrow();

            match *borrowed_pred_result {
                LispCell::Bool(true) => exec(env, true_case.clone()),
                LispCell::Bool(false) => exec(env, false_case.clone()),
                ref r @ _ => panic!("Invalid result returned by if predicate: {:?}", r),
            }
        }
        _ => panic!("Invalid arg num passed to if: {:?}", &args),
    }
}

pub fn eq(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [left, right] => {
            let is_eq = left == right;

            Rc::new(RefCell::new(LispCell::Bool(is_eq)))
        }
        _ => panic!("Invalid arg num passed to if: {:?}", &args),
    }
}

fn to_nums<'a>(args: &'a Vec<LispCellRef>) -> Box<Iterator<Item = f32> + 'a> {
    let map = args.iter().map(|arg| match *arg.borrow() {
        LispCell::Number(num) => num,
        ref c @ _ => panic!("Non-number cell handed to numeric operator: {:?}", c),
    });

    Box::new(map)
}

fn lisp_null() -> LispCellRef {
    LispCell::new_list(vec![])
}
