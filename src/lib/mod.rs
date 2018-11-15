pub mod core;
pub mod exec;
pub mod parse;
pub mod print;
pub mod util;

pub use core::*;
pub use exec::*;
pub use parse::*;
pub use print::*;
pub use util::*;

#[cfg(test)]
mod test {
    use super::core::{LispCell, LispProgram};
    use super::{exec_prog, new_env, parse, print};

    #[test]
    fn basic_parsing_and_printing() {
        let program_str = "(do (print (+ 1 2) (- 1 2)) (foo bar baz (qux (+ 1 2) (blah) blah)))";
        let program = parse(program_str.to_string());

        assert!(print(&program) == program_str, "Expected program_str and printed program to be equal");
    }

    #[test]
    fn basic_parsing() {
        let program_str = "(print (concat (+ 1 2) (- 3 5)))";
        let parsed_program = parse(program_str.to_string());

        let expected_program = LispProgram {
            text: program_str.to_string(),
            entry: Some(Box::new(LispCell::List {
                contents: vec![
                    make_atom("print"),
                    LispCell::List {
                        contents: vec![
                            make_atom("concat"),
                            LispCell::List {
                                contents: vec![make_atom("+"), make_num(1f32), make_num(2f32)],
                            },
                            LispCell::List {
                                contents: vec![make_atom("-"), make_num(3f32), make_num(5f32)],
                            },
                        ],
                    },
                ],
            })),
        };

        assert_eq!(parsed_program, expected_program, "Expected parsed program and expected program to be equal")
    }

    #[test]
    fn basic_list_parsing() {
        let program_str = "(print (+ 1 2) '(1 (+ 1 2)) (- 3 5))";
        let parsed_program = parse(program_str.to_string());

        let expected_program = LispProgram {
            text: program_str.to_string(),
            entry: Some(Box::new(LispCell::List {
                contents: vec![
                    make_atom("print"),
                    LispCell::List {
                        contents: vec![make_atom("+"), make_num(1f32), make_num(2f32)],
                    },
                    LispCell::Quoted(Box::new(LispCell::List {
                        contents: vec![
                            make_num(1f32),
                            LispCell::List {
                                contents: { vec![make_atom("+"), make_num(1f32), make_num(2f32)] },
                            },
                        ],
                    })),
                    LispCell::List {
                        contents: vec![make_atom("-"), make_num(3f32), make_num(5f32)],
                    },
                ],
            })),
        };

        assert_eq!(parsed_program, expected_program, "Expected parsed program and expected program to be equal")
    }

    #[test]
    fn basic_adding() {
        let program_str = "(+ 1 2)".to_string();
        let program = parse(program_str);

        let env = new_env();
        let result = exec_prog(env, program);
        println!("result: {:?}", result);

        assert_eq!(result, make_num(3f32));
    }

    #[test]
    fn basic_adding_2() {
        let program_str = "(+ (+ 1 2) (+ 2 2))".to_string();
        let program = parse(program_str);

        let env = new_env();
        let result = exec_prog(env, program);
        println!("result: {:?}", result);

        assert_eq!(result, make_num(7f32));
    }

    #[test]
    fn basic_math() {
        let program_str = "(* (+ (* 1 2 3) (- 2 2 -5) (+ 1 1 2 3) (/ 1 2)) (+ 1 5 6))".to_string();
        let program = parse(program_str);

        let env = new_env();
        let result = exec_prog(env, program);
        println!("result: {:?}", result);

        assert_eq!(result, make_num(222f32));
    }

    fn make_num(num: f32) -> LispCell {
        LispCell::Number(num)
    }

    fn make_atom(name: &'static str) -> LispCell {
        LispCell::Atom(name.to_string())
    }
}
