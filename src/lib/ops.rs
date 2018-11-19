use std::cell::RefCell;
use std::rc::Rc;

use super::core::log;
use super::util;
use super::{exec, Environment, LispCell, LispCellRef};

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
    LispCell::new_list(&args.clone())
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

pub fn dew(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    // Execute each arg in the vec and return the last expr result as the result
    args.iter().map(|arg| exec(env, arg.clone())).last().unwrap()
}

pub fn push(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [el, list] => match *list.borrow_mut() {
            LispCell::List {
                ref contents,
            } => {
                contents.borrow_mut().push_back(el.clone());

                list.clone()
            }
            ref l @ _ => panic!("Second arg passed to push was not a list: {:?}", l),
        },
        _ => panic!("Invalid arg num passed to push: {:?}", &args),
    }
}

pub fn car(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [list] => match *list.borrow() {
            LispCell::List {
                ref contents,
            } => match contents.borrow().front() {
                Some(first) => first.clone(),
                None => lisp_null(),
            },
            ref l @ _ => panic!("Arg passed to push was not a list: {:?}", l),
        },
        _ => panic!("Invalid arg num passed to push: {:?}", &args),
    }
}

pub fn cdr(env: &mut Environment, args: &Vec<LispCellRef>) -> LispCellRef {
    match args.as_slice() {
        [list] => match *list.borrow() {
            LispCell::List {
                ref contents,
            } => {
                let mut borrowed_contents = contents.borrow_mut();
                println!("cdr'ing: {:?}", &borrowed_contents);

                unsafe {
                    let (_, rest) = util::split_at_head(&mut borrowed_contents);

                    println!("borrowed_contents after split: {:?}", &borrowed_contents);
                    println!("rest: {:?}", &rest);

                    match rest.len() {
                        0 => lisp_null(),
                        _ => LispCell::make_list(rest),
                    }
                }
            }
            ref l @ _ => panic!("Arg passed to push was not a list: {:?}", l),
        },
        _ => panic!("Invalid arg num passed to push: {:?}", &args),
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
    LispCell::new_list(&vec![])
}
