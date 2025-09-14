use std::env;
use std::io;
use std::process;

use crate::pattern::RegexAst;
use crate::pattern::Repetition;
use crate::utils::pattern_to_ast;
mod pattern;
mod utils;


fn solve_ast(input_chars: &[char], node: &RegexAst, input_ind: usize) -> Vec<usize> {
    // Returns a list of indices where this node could end successfully
    match node {
        RegexAst::Literal(c) => {
            if input_ind < input_chars.len() && input_chars[input_ind] == *c {
                vec![input_ind + 1]
            } else {
                vec![]
            }
        }
        RegexAst::Digit => {
            if input_ind < input_chars.len() && input_chars[input_ind].is_ascii_digit() {
                vec![input_ind + 1]
            } else {
                vec![]
            }
        }
        RegexAst::Word => {
            if input_ind < input_chars.len() && (input_chars[input_ind].is_ascii_alphanumeric() || input_chars[input_ind] == '_') {
                vec![input_ind + 1]
            } else {
                vec![]
            }
        }
        RegexAst::Wildcard => {
            if input_ind < input_chars.len() {
                vec![input_ind + 1]
            } else {
                vec![]
            }
        }
        RegexAst::PositiveGroup(group) => {
            if input_ind < input_chars.len() && group.contains(input_chars[input_ind]) {
                vec![input_ind + 1]
            } else {
                vec![]
            }
        }
        RegexAst::NegativeGroup(group) => {
            if input_ind < input_chars.len() && !group.contains(input_chars[input_ind]) {
                vec![input_ind + 1]
            } else {
                vec![]
            }
        }
        RegexAst::StartOfLine => vec![input_ind], // handled externally
        RegexAst::EndOfLine => {
            if input_ind == input_chars.len() {
                vec![input_ind]
            } else {
                vec![]
            }
        }
        RegexAst::Concat(nodes) => {
            let mut indices = vec![input_ind];
            for sub in nodes {
                let mut new_indices = vec![];
                for &i in &indices {
                    new_indices.extend(solve_ast(input_chars, sub, i));
                }
                indices = new_indices;
                if indices.is_empty() {
                    break;
                }
            }
            indices
        }
        RegexAst::Alternate(nodes) => {
            let mut indices = vec![];
            for sub in nodes {
                indices.extend(solve_ast(input_chars, sub, input_ind));
            }
            indices
        }
        RegexAst::Repeat(sub, rep) => match rep {
            Repetition::None => solve_ast(input_chars, sub, input_ind),
            Repetition::Plus => {
                // one or more
                let mut result = vec![];
                for i in solve_ast(input_chars, sub, input_ind) {
                    // keep repeating
                    let mut more = solve_ast_repeat(input_chars, sub, i, true);
                    result.append(&mut more);
                }
                result
            }
            Repetition::Star => {
                // zero or more
                let mut result = vec![input_ind];
                result.append(&mut solve_ast_repeat(input_chars, sub, input_ind, false));
                result
            }
        },
    }
}

fn solve_ast_repeat(input_chars: &[char], sub: &RegexAst, input_ind: usize, must_have_one: bool) -> Vec<usize> {
    let mut results = vec![];
    let mut queue = vec![input_ind];

    while let Some(ind) = queue.pop() {
        let next = solve_ast(input_chars, sub, ind);
        for &i in &next {
            if i > ind {
                results.push(i);
                queue.push(i);
            }
        }
    }

    if must_have_one && results.is_empty() {
        vec![]
    } else {
        results
    }
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let ast = pattern_to_ast(pattern);
    eprintln!("{:?}", ast);
    let input_chars: Vec<char> = input_line.trim_end().chars().collect();
    !solve_ast(&input_chars, &ast, 0).is_empty()
}


// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
