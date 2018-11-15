use super::{Environment, LispCell, exec};

pub fn add(env: &mut Environment, args: &Vec<LispCell>) -> LispCell {
    LispCell::Number(to_nums(args).sum())
}

pub fn sub(env: &mut Environment, args: &Vec<LispCell>) -> LispCell {
    let mut nums = to_nums(args);
    let first = nums.next().unwrap();

    LispCell::Number(nums.fold(first, |acc, val| acc - val))
}

pub fn mul(env: &mut Environment, args: &Vec<LispCell>) -> LispCell {
    LispCell::Number(to_nums(args).product())
}

pub fn div(env: &mut Environment, args: &Vec<LispCell>) -> LispCell {
    let mut nums = to_nums(args);
    let first = nums.next().unwrap();

    LispCell::Number(nums.fold(first, |acc, val| acc / val))
}

pub fn def(env: &mut Environment, args: &Vec<LispCell>) -> LispCell {
    let mut iter = args.iter();

    let cell = iter.next().unwrap();
    match cell {
        LispCell::Atom(symbol) => {
            let value = exec(env, iter.next().unwrap());

            env.def(symbol.clone(), value);

            cell.clone()
        },
        _ => panic!("")
    }
}

pub fn do_fn(env: &mut Environment, args: &Vec<LispCell>) -> LispCell {
    // Execute each arg in the vec and return the last expr result as the result
    args.iter().map(|arg| exec(env, arg)).last().unwrap()
}

fn to_nums<'a>(args: &'a Vec<LispCell>) -> Box<Iterator<Item = f32> + 'a> {
    let map = args.iter().map(|arg| match arg {
        LispCell::Number(num) => *num,
        c @ _ => panic!("Non-number cell handed to numeric operator: {:?}", c),
    });

    Box::new(map)
}
