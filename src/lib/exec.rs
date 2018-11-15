use super::*;
use std::collections::HashMap;

pub fn exec(env: Environment, mut program: LispProgram) -> LispCell {
    match program.entry {
        Some(e) => exec_rec(&env, &*e),
        _ => panic!("No entry found for program!"),
    }
}

pub fn exec_rec(env: &Environment, cell: &LispCell) -> LispCell {
    match cell {
        c @ LispCell::Atom(_) => c.clone(),
        LispCell::List {
            contents,
        } => {
            let (x, xs) = contents.split_at(1);

            let operator = x.get(0).unwrap();
            let operands = xs.clone().iter().map(|cell| exec_rec(env, cell)).collect();

            call_fn(&env, operator, &operands)
        }
        t @ _ => panic!("LispCell type {:?} not implemented!", t),
    }
}

fn call_fn(env: &Environment, operator: &LispCell, operands: &Vec<LispCell>) -> LispCell {
    match operator {
        LispCell::Atom(atom) => match env.operators.get(atom.as_str()) {
            Some(op) => op(operands),
            None => panic!("Operator {:?} not implemented", atom),
        },
        t @ _ => panic!("LispCell type {:?} not implemented!", t),
    }
}

pub type OperatorFn = Fn(&Vec<LispCell>) -> LispCell;

pub struct Environment<'a> {
    operators: HashMap<&'a str, Box<OperatorFn>>,
}

pub fn new_env<'a>() -> Environment<'a> {
    Environment {
        operators: make_operators(),
    }
}

fn make_operators<'a>() -> HashMap<&'a str, Box<OperatorFn>> {
    let mut map: HashMap<&'a str, Box<OperatorFn>> = HashMap::new();
    map.insert(
        "+",
        Box::new(move |ops| {
            let sum = ops
                .iter()
                .map(|op| match op {
                    LispCell::Atom(atom) => atom.parse::<f32>().unwrap(),
                    c @ _ => panic!("Non-atom cell handed to +: {:?}", c),
                }).fold(0f32, |acc, val| acc + val);

            LispCell::Atom(sum.to_string())
        }),
    );

    map
}
