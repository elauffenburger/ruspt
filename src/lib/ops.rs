use super::core::log;
use std::rc::Rc;

use super::{exec, Environment, LispCell};

pub fn add(env: &mut Environment, args: &Vec<Rc<LispCell>>) -> Rc<LispCell> {
    Rc::new(LispCell::Number(to_nums(args).sum()))
}

pub fn sub(env: &mut Environment, args: &Vec<Rc<LispCell>>) -> Rc<LispCell> {
    let mut nums = to_nums(args);
    let first = nums.next().unwrap();

    Rc::new(LispCell::Number(nums.fold(first, |acc, val| acc - val)))
}

pub fn mul(env: &mut Environment, args: &Vec<Rc<LispCell>>) -> Rc<LispCell> {
    Rc::new(LispCell::Number(to_nums(args).product()))
}

pub fn div(env: &mut Environment, args: &Vec<Rc<LispCell>>) -> Rc<LispCell> {
    let mut nums = to_nums(args);
    let first = nums.next().unwrap();

    Rc::new(LispCell::Number(nums.fold(first, |acc, val| acc / val)))
}

pub fn def(env: &mut Environment, args: &Vec<Rc<LispCell>>) -> Rc<LispCell> {
    let mut iter = args.iter();

    let cell = iter.next().unwrap();
    match *cell.clone() {
        LispCell::Atom(ref symbol) => {
            let value = exec(env, iter.next().unwrap());

            log(|| println!("Defining symbol: {:?} with value: {:?}", symbol, value));

            env.def(symbol.clone(), value);

            log(|| println!("Symbol {:?} defined", symbol));

            cell.clone()
        }
        _ => panic!("Unable to find symbol to define in call to def"),
    }
}

pub fn do_fn(env: &mut Environment, args: &Vec<Rc<LispCell>>) -> Rc<LispCell> {
    // Execute each arg in the vec and return the last expr result as the result
    args.iter().map(|arg| exec(env, arg)).last().unwrap()
}

fn to_nums<'a>(args: &'a Vec<Rc<LispCell>>) -> Box<Iterator<Item = f32> + 'a> {
    let map = args.iter().map(|arg| match *arg.clone() {
        LispCell::Number(num) => num,
        ref c @ _ => panic!("Non-number cell handed to numeric operator: {:?}", c),
    });

    Box::new(map)
}