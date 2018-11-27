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
    use std::cell::RefCell;
    use std::rc::Rc;

    use print::print_cell;

    use super::core::{Environment, LispCellRef, LispProgram};
    use super::{exec_prog, parse, print};

    use super::util::*;

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
                make_quoted(make_list(vec![
                    make_num(1f32),
                    make_list(vec![make_atom("+"), make_num(1f32), make_num(2f32)]),
                ])),
                make_list(vec![make_atom("-"), make_num(3f32), make_num(5f32)]),
            ])),
        };

        assert_eq!(parsed_program, expected_program, "Expected parsed program and expected program to be equal")
    }

    #[test]
    fn parse_empty_list() {
        let program_str = "(print ())";
        let parsed_program = parse(program_str.to_string());

        let expected_program = LispProgram {
            text: program_str.to_string(),
            entry: Some(make_list(vec![make_atom("print"), make_list(vec![])])),
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
        run_exec_test_literal("(do (def x (list 1)) (push 3 x) (push 2 x) x)", "(1 2 3)")
    }

    #[test]
    fn car() {
        run_exec_test_literal("(do (def x (list (list 2 4) 0 1)) (push 3 (car x)) x)", "((2 3 4) 0 1)")
    }

    #[test]
    fn cdr() {
        run_exec_test_literal("(do (def x (list (list 4 5 6) 1 3)) (push 2 (cdr x)) x)", "((4 5 6) 1 2 3)")
    }

    #[test]
    fn iff() {
        run_exec_test_literal("(do (def x (if (eq 1 1) 1 0)) x)", "1")
    }

    #[test]
    fn basic_defn() {
        run_exec_test_literal("(do (defn foo () (+ 1 1)) (foo))", "2")
    }

    #[test]
    fn basic_defn_with_args() {
        run_exec_test_literal("(do (def x 5) (defn foo (x) (+ x 1)) (foo x))", "6")
    }

    #[test]
    fn basic_lambda() {
        run_exec_test_literal("(do ((lambda () (* 3 2))))", "6")
    }

    #[test]
    fn basic_lambda_with_args() {
        run_exec_test_literal("(do ((lambda (x) (* x 3)) 3))", "9")
    }

    #[test]
    fn basic_lambda_with_args_and_def() {
        run_exec_test_literal("(do (def x (lambda (x) (* x 3))) (x 3))", "9")
    }

    #[test]
    fn nested_env() {
        run_exec_test_literal("(do (def x 3) (def f (lambda (y) (* x y))) (f 3))", "9")
    }

    #[test]
    fn original_var_is_unaltered_by_shadow() {
        run_exec_test_literal("(do (def x 3) (def f (lambda (x) (* x 2))) (+ (f 12) x)", "27")
    }

    fn run_exec_test_literal<'a, 'b>(prog_str: &'a str, expected_result_str: &'b str) {
        let expected_result = parse(expected_result_str.to_string()).entry.unwrap();

        run_exec_test(prog_str, expected_result)
    }

    fn run_exec_test<'a>(prog_str: &'a str, expected_result: LispCellRef) {
        let program = parse(prog_str.to_string());

        let mut env = Rc::new(RefCell::new(Environment::new()));
        let result = exec_prog(env, program);
        println!("result: {:?}", &result);
        println!("pretty result: {:?}", print_cell(result.clone()));

        assert_eq!(*result, *expected_result);
    }
}
