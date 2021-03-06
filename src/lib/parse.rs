use core::*;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy, Debug)]
enum ParseMode {
    Normal,
    InStr,
}

pub fn parse(program: String) -> LispProgram {
    let mut trimmed_program = program.trim().to_string();
    log(|| println!("program: {}", &trimmed_program));

    let entry = parse_init(&mut trimmed_program);

    LispProgram {
        text: trimmed_program,
        entry: Some(entry),
    }
}

fn parse_init(program: &mut String) -> LispCellRef {
    let mut sanitized_program = program.replace("(", " ( ").replace(")", " ) ");
    log(|| println!("sanitized_program: {:?}", &sanitized_program));

    let mut results = vec![];
    parse_rec(&mut sanitized_program, true, ParseMode::Normal, &mut vec![], &mut String::new(), &mut results, 0);

    log(|| println!("results: {:?}", &results));

    return results.pop().unwrap();
}

fn parse_rec(
    text: &mut String,
    greedy: bool,
    mode: ParseMode,
    list_stack: &mut Vec<char>,
    pending_word: &mut String,
    results: &mut Vec<LispCellRef>,
    depth: i32,
) {
    log(|| println!("{}results: {:?}", tab_to_depth(depth), &results));

    if text.is_empty() {
        if pending_word != "" {
            parse_rec_finalize_word(text, false, mode, list_stack, pending_word, results, depth);
        }

        return;
    }

    match text.remove(0) {
        ' ' | '\n' => {
            log(|| println!("{}in whitespace", tab_to_depth(depth)));

            parse_rec_finalize_word(text, greedy, mode, list_stack, pending_word, results, depth);
        }
        '\'' => {
            log(|| println!("{}in '", tab_to_depth(depth)));

            let mut to_quote = vec![];
            parse_rec(text, false, mode, list_stack, &mut String::new(), &mut to_quote, depth);

            log(|| println!("{}to quote: {:?}", tab_to_depth(depth), &to_quote));

            results.push(Rc::new(RefCell::new(LispCell::Quoted(to_quote.pop().unwrap()))));

            if greedy {
                parse_rec(text, greedy, mode, list_stack, &mut String::new(), results, depth);
            }
        }
        '(' => {
            log(|| println!("{}in (", tab_to_depth(depth)));

            list_stack.push('(');

            log(|| println!("{}Staring new results stack", tab_to_depth(depth)));

            let mut list_contents = vec![];
            parse_rec(text, true, mode, list_stack, &mut String::new(), &mut list_contents, depth + 1);

            log(|| println!("{}Finished results stack: {:?}", tab_to_depth(depth), &list_contents));

            results.push(LispCell::new_list(list_contents));

            if greedy {
                parse_rec(text, greedy, mode, list_stack, &mut String::new(), results, depth);
            }
        }
        ')' => {
            log(|| println!("{}in )", tab_to_depth(depth)));

            if list_stack.is_empty() {
                // TODO: handle this better
                panic!("Invalid program: unmatched parens");
            }

            list_stack.pop();
        }
        '"' => match mode {
            ParseMode::InStr => {
                results.push(LispCell::Str(pending_word.clone()).to_ref());

                if greedy {
                    parse_rec(text, greedy, ParseMode::Normal, list_stack, &mut String::from(""), results, depth)
                }
            }
            _ => parse_rec(text, greedy, ParseMode::InStr, list_stack, pending_word, results, depth),
        },
        c @ _ => {
            log(|| println!("{}c: {}", tab_to_depth(depth), &c));

            // We're either starting or adding to a pending word
            pending_word.push(c);

            // ...either way, we just continue
            parse_rec(text, greedy, mode, list_stack, pending_word, results, depth)
        }
    }
}

fn parse_rec_finalize_word(
    text: &mut String,
    greedy: bool,
    mode: ParseMode,
    list_stack: &mut Vec<char>,
    pending_word: &mut String,
    results: &mut Vec<LispCellRef>,
    depth: i32,
) {
    // If there's no pending_word, just move onto the next char
    if pending_word == "" {
        return parse_rec(text, greedy, mode, list_stack, pending_word, results, depth);
    }

    log(|| println!("{}finalizing word: {}", tab_to_depth(depth), &pending_word));

    let pending_word_str = pending_word.to_string();
    let cell = match pending_word_str.parse::<f32>() {
        Ok(num) => LispCell::Number(num),
        _ => LispCell::Atom(pending_word_str),
    };

    // Otherwise, close out this word and add it to the result set
    results.push(Rc::new(RefCell::new(cell)));

    if greedy {
        // Move onto the next char
        parse_rec(text, greedy, mode, list_stack, &mut String::new(), results, depth);
    }
}

fn log<F>(log_fn: F)
where
    F: FnOnce(),
{
    if cfg!(feature = "parse_debug") {
        log_fn();
    }
}

fn tab_to_depth(depth: i32) -> String {
    format!("({}): {}", depth, "  ".repeat(depth as usize).to_string())
}
