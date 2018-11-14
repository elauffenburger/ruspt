use core::*;

pub fn print(program: &LispProgram) -> String {
    match program.entry {
        None => "".to_string(),
        Some(ref entry) => {
            let mut result = String::new();
            print_rec(entry, &mut result);

            return result;
        }
    }
}

fn print_rec(node: &LispCell, result: &mut String) {
    match node {
        LispCell::Atom(atom) => {
            result.push_str(atom.as_str());
        }
        LispCell::List { contents } => {
            result.push('(');

            let n = contents.len();
            for i in 0..n {
                let cell = &contents[i];

                print_rec(cell, result);

                if i != n - 1 {
                    result.push(' ');
                }
            }

            result.push(')');
        }
        LispCell::Func { operator, operands } => {
            result.push('(');

            print_rec(operator, result);

            for ref operand in operands {
                result.push(' ');

                print_rec(operand, result);
            }

            result.push(')');
        }
    }
}