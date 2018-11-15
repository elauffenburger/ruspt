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

    LispProgram {
        text: trimmed_program,
        entry: Some(Box::new(entry)),
    }
}

fn parse_init(program: &mut String) -> LispCell {
    let mut sanitized_program = program.replace("(", " ( ").replace(")", " ) ");
    log(|| println!("sanitized_program: {:?}", &sanitized_program));

    let mut results = vec![];
    parse_rec(&mut sanitized_program, true, &mut vec![], &mut String::new(), &mut results, 0);

    println!("results: {:?}", &results);

    return results.pop().unwrap();
}

fn parse_rec(text: &mut String, greedy: bool, list_stack: &mut Vec<char>, pending_word: &mut String, results: &mut Vec<LispCell>, depth: i32) {
    log(|| println!("{}results: {:?}", tab_to_depth(depth), &results));

    if text.is_empty() {
        // let contents = results.clone();
        // results.push(LispCell::List {
        //     contents: contents,
        // });

        return;
    }

    match text.remove(0) {
        ' ' | '\n' => {
            log(|| println!("{}in whitespace", tab_to_depth(depth)));

            // If there's no pending_word, just move onto the next char
            if pending_word == "" {
                return parse_rec(text, greedy, list_stack, pending_word, results, depth);
            }

            log(|| println!("{}finalizing word: {}", tab_to_depth(depth), &pending_word));

            let pending_word_str = pending_word.to_string();

            // Otherwise, close out this word and add it to the result set
            results.push(LispCell::Atom(pending_word_str));

            if greedy {
                // Move onto the next char
                parse_rec(text, greedy, list_stack, &mut String::new(), results, depth);
            }
        }
        '\'' => {
            log(|| println!("{}in '", tab_to_depth(depth)));

            let mut to_quote = vec![];
            parse_rec(text, false, list_stack, &mut String::new(), &mut to_quote, depth);

            log(|| println!("{}to quote: {:?}", tab_to_depth(depth), &to_quote));

            results.push(LispCell::Quoted(Box::new(to_quote.pop().unwrap())));

            if greedy {
                parse_rec(text, greedy, list_stack, &mut String::new(), results, depth);
            }
        }
        '(' => {
            log(|| println!("{}in (", tab_to_depth(depth)));

            list_stack.push('(');

            log(|| println!("{}Staring new results stack", tab_to_depth(depth)));

            let mut list_contents = vec![];
            parse_rec(text, true, list_stack, &mut String::new(), &mut list_contents, depth + 1);

            log(|| println!("{}Finished results stack: {:?}", tab_to_depth(depth), &list_contents));

            results.push(LispCell::List {
                contents: list_contents,
            });

            if greedy {
                parse_rec(text, greedy, list_stack, &mut String::new(), results, depth);
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
        '"' => {
            // TODO: handle, you know, strings
            panic!("Strings not supported yet!")
        }
        c @ _ => {
            log(|| println!("{}c: {}", tab_to_depth(depth), &c));

            // We're either starting or adding to a pending word
            pending_word.push(c);

            // ...either way, we just continue
            parse_rec(text, greedy, list_stack, pending_word, results, depth)
        }
    }
}

fn tab_to_depth(depth: i32) -> String {
    format!("({}): {}", depth, "  ".repeat(depth as usize).to_string())
}
