use std::rc::Rc;
use std::cell::RefCell;

use super::core::log;
use super::{exec, Environment, LispCell};

pub fn add(env: &mut Environment, args: &Vec<Rc<RefCell<LispCell>>>) -> Rc<RefCell<LispCell>> {
    Rc::new(RefCell::new(LispCell::Number(to_nums(args).sum())))
}

pub fn sub(env: &mut Environment, args: &Vec<Rc<RefCell<LispCell>>>) -> Rc<RefCell<LispCell>> {
    let mut nums = to_nums(args);
    let first = nums.next().unwrap();

    Rc::new(RefCell::new(LispCell::Number(nums.fold(first, |acc, val| acc - val))))
}

pub fn mul(env: &mut Environment, args: &Vec<Rc<RefCell<LispCell>>>) -> Rc<RefCell<LispCell>> {
    Rc::new(RefCell::new(LispCell::Number(to_nums(args).product())))
}

pub fn div(env: &mut Environment, args: &Vec<Rc<RefCell<LispCell>>>) -> Rc<RefCell<LispCell>> {
    let mut nums = to_nums(args);
    let first = nums.next().unwrap();

    Rc::new(RefCell::new(LispCell::Number(nums.fold(first, |acc, val| acc / val))))
}

pub fn def(env: &mut Environment, args: &Vec<Rc<RefCell<LispCell>>>) -> Rc<RefCell<LispCell>> {
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

pub fn dew(env: &mut Environment, args: &Vec<Rc<RefCell<LispCell>>>) -> Rc<RefCell<LispCell>> {
    // Execute each arg in the vec and return the last expr result as the result
    args.iter().map(|arg| exec(env, arg.clone())).last().unwrap()
}

pub fn push(env: &mut Environment, args: &Vec<Rc<RefCell<LispCell>>>) -> Rc<RefCell<LispCell>> {
    match args.as_slice() {
        [el, list] => {
            match *list.borrow_mut() {
                LispCell::List{ ref mut contents } => {
                    contents.push(el.clone());

                    list.clone()
                },
                ref l @ _ => panic!("Second arg passed to push was not a list: {:?}", l)
            }
        },
        _ => panic!("Invalid args passed to push")
    }
}

fn to_nums<'a>(args: &'a Vec<Rc<RefCell<LispCell>>>) -> Box<Iterator<Item = f32> + 'a> {
    let map = args.iter().map(|arg| match *arg.borrow() {
        LispCell::Number(num) => num,
        ref c @ _ => panic!("Non-number cell handed to numeric operator: {:?}", c),
    });

    Box::new(map)
}