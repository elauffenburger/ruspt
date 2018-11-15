pub mod core;
pub mod display;
pub mod exec;
pub mod parse;
pub mod util;

pub use core::*;
pub use display::*;
pub use exec::*;
pub use parse::*;
pub use util::*;

#[cfg(test)]
mod test {
    use super::core::{LispCell, LispProgram};
    use super::{exec, new_env, parse, print};

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
                                contents: vec![make_atom("+"), make_atom("1"), make_atom("2")],
                            },
                            LispCell::List {
                                contents: vec![make_atom("-"), make_atom("3"), make_atom("5")],
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
                        contents: vec![make_atom("+"), make_atom("1"), make_atom("2")],
                    },
                    LispCell::Quoted(Box::new(LispCell::List {
                        contents: vec![
                            make_atom("1"),
                            LispCell::List {
                                contents: { vec![make_atom("+"), make_atom("1"), make_atom("2")] },
                            },
                        ],
                    })),
                    LispCell::List {
                        contents: vec![make_atom("-"), make_atom("3"), make_atom("5")],
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
        let result = exec(env, program);
        println!("result: {:?}", result);

        assert_eq!(result, make_atom("3"));
    }

    #[test]
    fn basic_adding_2() {
        let program_str = "(+ (+ 1 2) (+ 2 2))".to_string();
        let program = parse(program_str);

        let env = new_env();
        let result = exec(env, program);
        println!("result: {:?}", result);

        assert_eq!(result, make_atom("7"));
    }

    fn make_atom(name: &'static str) -> LispCell {
        LispCell::Atom(name.to_string())
    }
}
