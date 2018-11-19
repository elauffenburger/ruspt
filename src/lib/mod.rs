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

    use super::core::{LispCell, LispCellRef, LispProgram};
    use super::{exec_prog, new_env, parse, print};

    #[test]
    fn basic_parsing_and_printing() {
        let program_str = "(do (print (+ 1 2) (- 1 2)) (foo bar baz (qux (+ 1 2) (blah) blah)))";
        let program = parse(program_str.to_string());

        assert_eq!(print(&program), program_str, "Expected program_str and printed program to be equal");
    }

    #[test]
    fn basic_parsing() {
        let program_str = "(print (concat (+ 1 2) (- 3 5)))";
        let parsed_program = parse(program_str.to_string());

        let expected_program = LispProgram {
            text: program_str.to_string(),
            entry: Some(make_list(vec![
                make_atom("print"),
                make_list(vec![
                    make_atom("concat"),
                    make_list(vec![make_atom("+"), make_num(1f32), make_num(2f32)]),
                    make_list(vec![make_atom("-"), make_num(3f32), make_num(5f32)]),
                ]),
            ])),
        };

        assert_eq!(parsed_program, expected_program, "Expected parsed program and expected program to be equal")
    }

    #[test]
    fn basic_list_parsing() {
        let program_str = "(print (+ 1 2) '(1 (+ 1 2)) (- 3 5))";
        let parsed_program = parse(program_str.to_string());

        let expected_program = LispProgram {
            text: program_str.to_string(),
            entry: Some(make_list(vec![
                make_atom("print"),
                make_list(vec![make_atom("+"), make_num(1f32), make_num(2f32)]),
                make_quoted(make_list(vec![make_num(1f32), make_list(vec![make_atom("+"), make_num(1f32), make_num(2f32)])])),
                make_list(vec![make_atom("-"), make_num(3f32), make_num(5f32)]),
            ])),
        };

        assert_eq!(parsed_program, expected_program, "Expected parsed program and expected program to be equal")
    }

    #[test]
    fn basic_adding() {
        run_exec_test("(+ 1 2)", make_num(3f32))
    }

    #[test]
    fn basic_adding_2() {
        run_exec_test("(+ (+ 1 2) (+ 2 2))", make_num(7f32))
    }

    #[test]
    fn basic_math() {
        run_exec_test("(* (+ (* 1 2 3) (- 2 2 -5) (+ 1 1 2 3) (/ 1 2)) (+ 1 5 6))", make_num(222f32))
    }

    #[test]
    fn basic_list() {
        run_exec_test("(list 1 2 3)", make_list(vec![make_num(1f32), make_num(2f32), make_num(3f32)]))
    }

    #[test]
    fn basic_def_and_do() {
        run_exec_test("(do (def x (+ 2 2)) (+ x 5))", make_num(9f32))
    }

    #[test]
    fn basic_def_and_push() {
        run_exec_test_literal("(do (def x (list 1 2 3)) (push 4 x) (push 5 x) x)", "(1 2 3 4 5)")
    }

    #[test]
    fn car() {
        run_exec_test_literal("(do (def x (list (list 3 4 5) 1 2)) (push 6 (car x)) x)", "((3 4 5 6) 1 2)")
    }

    #[test]
    fn cdr() {
        run_exec_test_literal("(do (def x (list (list 3 4 5) 1 2)) (push 3 (cdr x)) x)", "((3 4 5) 1 2 3)")
    }

    fn run_exec_test_literal<'a, 'b>(prog_str: &'a str, expected_result_str: &'b str) {
        let expected_result = parse(expected_result_str.to_string()).entry.unwrap();

        run_exec_test(prog_str, expected_result)
    }

    fn run_exec_test<'a>(prog_str: &'a str, expected_result: LispCellRef) {
        let program = parse(prog_str.to_string());

        let env = new_env();
        let result = exec_prog(env, program);
        println!("result: {:?}", &result);
        println!("pretty result: {:?}", print_cell(result.clone()));

        assert_eq!(*result, *expected_result);
    }

    fn make_num(num: f32) -> LispCellRef {
        Rc::new(RefCell::new(LispCell::Number(num)))
    }

    fn make_atom(name: &'static str) -> LispCellRef {
        Rc::new(RefCell::new(LispCell::Atom(name.to_string())))
    }

    fn make_list(list: Vec<LispCellRef>) -> LispCellRef {
        LispCell::new_list(&list)
    }

    fn make_quoted(cell: LispCellRef) -> LispCellRef {
        Rc::new(RefCell::new(LispCell::Quoted(cell)))
    }
}
