use core::*;
use util::*;

#[derive(Debug, Clone)]
enum ParseMode {
    Normal,
    InString,
    InFunc,
    InList,
}

pub fn parse(mut program: String) -> LispProgram {
    let mut trimmed_program = program.trim().to_string();
    log(|| println!("program: {}", &trimmed_program));

    let entry = parse_init(&mut trimmed_program);

    LispProgram { text: trimmed_program, entry: Some(Box::new(entry)) }
}

fn parse_init(program: &mut String) -> LispCell {
    let mut sanitized_program = program.replace("(", " ( ").replace(")", " ) ");
    log(|| println!("sanitized_program: {:?}", &sanitized_program));

    return parse_rec(&mut sanitized_program, ParseMode::Normal, &mut vec![], &mut String::new(), vec![]);
}

fn parse_rec(text: &mut String, mode: ParseMode, list_stack: &mut Vec<char>, pending_word: &mut String, mut results: Vec<LispCell>) -> LispCell {
    log(|| println!("mode: {:?}", mode));

    if text.is_empty() {
        return results.pop().unwrap();
    }

    match text.remove(0) {
        ' ' | '\n' => {
            log(|| println!("in whitespace"));

            // If there's no pending_word, just move onto the next char
            if pending_word == "" {
                return parse_rec(text, mode, list_stack, pending_word, results);
            }

            log(|| println!("finalizing word: {}", &pending_word));

            let pending_word_str = pending_word.to_string();

            // Otherwise, close out this word and add it to the result set
            results.push(LispCell::Atom(pending_word_str));

            // Move onto the next char
            parse_rec(text, mode, list_stack, &mut String::new(), results)
        }
        '\'' => {
            log(|| println!("in '"));

            parse_rec(text, ParseMode::InList, list_stack, pending_word, results)
        }
        '(' => {
            log(|| println!("in ("));

            // Keep track of being inside this list
            list_stack.push('(');

            // Get the contents of the list
            let mut list = parse_rec(text, ParseMode::InFunc, list_stack, &mut String::new(), vec![]);

            results.push(list);

            log(|| println!("results: {:?}", &results));

            // Move onto the next token in program
            parse_rec(text, mode, list_stack, &mut String::new(), results)
        }
        ')' => {
            log(|| println!("in )"));

            if list_stack.is_empty() {
                // TODO: handle this better
                panic!("Invalid program: unmatched parens");
            }

            list_stack.pop();

            log(|| println!("list contents: {:?}", &results));
            log(|| println!("mode: {:?}", &mode));

            match results.as_slice() {
                [] => LispCell::List { contents: vec![] },
                _ => match mode {
                    ParseMode::InFunc => {
                        let operator = results.remove(0);
                        LispCell::Func { operator: Box::new(operator), operands: results }
                    }
                    ParseMode::InList => LispCell::List { contents: results },
                    _ => panic!("Unsupported eval mode!"),
                },
            }
        }
        '"' => {
            // TODO: handle, you know, strings
            panic!("Strings not supported yet!")
        }
        c @ _ => {
            log(|| println!("c: {}", &c));

            // We're either starting or adding to a pending word
            pending_word.push(c);

            // ...either way, we just continue
            parse_rec(text, mode, list_stack, pending_word, results)
        }
    }
}
