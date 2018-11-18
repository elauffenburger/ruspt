pub mod core;
pub mod exec;
mod ops;
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
    use print::print_cell;
    use std::cell::RefCell;
    use std::rc::Rc;

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
            entry: Some(Rc::new(RefCell::new(LispCell::List {
                contents: vec![
                    make_atom("print"),
                    Rc::new(RefCell::new(LispCell::List {
                        contents: vec![
                            make_atom("concat"),
                            Rc::new(RefCell::new(LispCell::List {
                                contents: vec![make_atom("+"), make_num(1f32), make_num(2f32)],
                            })),
                            Rc::new(RefCell::new(LispCell::List {
                                contents: vec![make_atom("-"), make_num(3f32), make_num(5f32)],
                            })),
                        ],
                    })),
                ],
            }))),
        };

        assert_eq!(parsed_program, expected_program, "Expected parsed program and expected program to be equal")
    }

    #[test]
    fn basic_list_parsing() {
        let program_str = "(print (+ 1 2) '(1 (+ 1 2)) (- 3 5))";
        let parsed_program = parse(program_str.to_string());

        let expected_program = LispProgram {
            text: program_str.to_string(),
            entry: Some(Rc::new(RefCell::new(LispCell::List {
                contents: vec![
                    make_atom("print"),
                    Rc::new(RefCell::new(LispCell::List {
                        contents: vec![make_atom("+"), make_num(1f32), make_num(2f32)],
                    })),
                    Rc::new(RefCell::new(LispCell::Quoted(Rc::new(RefCell::new(LispCell::List {
                        contents: vec![
                            make_num(1f32),
                            Rc::new(RefCell::new(LispCell::List {
                                contents: { vec![make_atom("+"), make_num(1f32), make_num(2f32)] },
                            })),
                        ],
                    }))))),
                    Rc::new(RefCell::new(LispCell::List {
                        contents: vec![make_atom("-"), make_num(3f32), make_num(5f32)],
                    })),
                ],
            }))),
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

        assert_eq!(*result, *make_num(3f32));
    }

    #[test]
    fn basic_adding_2() {
        let program_str = "(+ (+ 1 2) (+ 2 2))".to_string();
        let program = parse(program_str);

        let env = new_env();
        let result = exec_prog(env, program);
        println!("result: {:?}", result);

        assert_eq!(*result, *make_num(7f32));
    }

    #[test]
    fn basic_math() {
        let program_str = "(* (+ (* 1 2 3) (- 2 2 -5) (+ 1 1 2 3) (/ 1 2)) (+ 1 5 6))".to_string();
        let program = parse(program_str);

        let env = new_env();
        let result = exec_prog(env, program);
        println!("result: {:?}", result);

        assert_eq!(*result, *make_num(222f32));
    }

    #[test]
    fn basic_list() {
        let program_str = "(list 1 2 3)".to_string();
        let program = parse(program_str);

        let env = new_env();
        let result = exec_prog(env, program);
        println!("result: {:?}", result);

        assert_eq!(*result, *make_list(vec![make_num(1f32), make_num(2f32), make_num(3f32)]));
    }

    #[test]
    fn basic_def_and_do() {
        let program_str = "(do (def x (+ 2 2)) (+ x 5))".to_string();
        let program = parse(program_str);

        let env = new_env();
        let result = exec_prog(env, program);
        println!("result: {:?}", result);

        assert_eq!(*result, *make_num(9f32));
    }

    #[test]
    fn basic_def_and_push() {
        let program_str = "(do (def x (list 1 2 3)) (push '4 x) (push '5 x) x)".to_string();
        let program = parse(program_str);

        let env = new_env();
        let result = exec_prog(env, program);
        println!("result: {:?}", result);

        assert_eq!(*result, *make_list(vec![make_num(1f32), make_num(2f32), make_num(3f32), make_num(4f32), make_num(5f32)]));
    }

    #[test]
    fn car() {
        let program_str = "(do (def x (list (list 3 4 5) 1 2)) (push '6 (car x)) x)".to_string();
        let program = parse(program_str);

        let env = new_env();
        let result = exec_prog(env, program);
        println!("result: {:?}", result);
        println!("\npretty result: {:?}", print_cell(result.clone()));

        assert_eq!(*result, *make_list(vec![make_list(vec![make_num(3f32), make_num(4f32), make_num(5f32), make_num(6f32)]), make_num(1f32), make_num(2f32)]));
    }

    fn make_num(num: f32) -> Rc<RefCell<LispCell>> {
        Rc::new(RefCell::new(LispCell::Number(num)))
    }

    fn make_atom(name: &'static str) -> Rc<RefCell<LispCell>> {
        Rc::new(RefCell::new(LispCell::Atom(name.to_string())))
    }

    fn make_list(list: Vec<Rc<RefCell<LispCell>>>) -> Rc<RefCell<LispCell>> {
        Rc::new(RefCell::new(LispCell::List {
            contents: list,
        }))
    }
}
