use std::env;
use std::io;
use std::process;

use crate::pattern::RegexAst;
use crate::pattern::Repetition;
use crate::utils::match_pattern_with_char;
use crate::utils::pattern_to_ast;
mod old_main;
mod pattern;
mod utils;

use std::collections::HashSet;


// Returns all possible end positions after matching this node starting from input_ind
fn solve(input_chars: &[char], node: &RegexAst, input_ind: usize) -> Vec<usize> {
    match node {
        // Single character matchers
        RegexAst::Digit
        | RegexAst::Word
        | RegexAst::PositiveGroup(_)
        | RegexAst::NegativeGroup(_)
        | RegexAst::Literal(_)
        | RegexAst::Wildcard => {
            if input_ind < input_chars.len() && match_pattern_with_char(node, input_chars[input_ind])
            {
                vec![input_ind + 1]
            } else {
                vec![]
            }
        }

        RegexAst::StartOfLine => {
            if input_ind == 0 {
                vec![0] // Matches at start, consumes no characters
            } else {
                vec![]
            }
        }

        RegexAst::EndOfLine => {
            if input_ind == input_chars.len() {
                vec![input_ind] // Matches at end, consumes no characters
            } else {
                vec![]
            }
        }

        RegexAst::Alternate(regex_asts) => {
            let mut results = HashSet::new();
            for option in regex_asts {
                for end_pos in solve(input_chars, option, input_ind) {
                    results.insert(end_pos);
                }
            }
            results.into_iter().collect()
        }

        RegexAst::Concat(regex_asts) => {
            let mut current_positions = vec![input_ind];

            for ast in regex_asts {
                let mut next_positions = HashSet::new();
                for &pos in &current_positions {
                    for end_pos in solve(input_chars, ast, pos) {
                        next_positions.insert(end_pos);
                    }
                }
                current_positions = next_positions.into_iter().collect();
                if current_positions.is_empty() {
                    break; // Early termination if no matches possible
                }
            }
            current_positions
        }


        RegexAst::CaptureGroup(group_id, ast) => {
            todo!()
        }

        RegexAst::Repeat(regex_ast, repetition) => {
            match repetition {
                Repetition::None => solve(input_chars, regex_ast, input_ind),

                Repetition::Optional => {
                    let mut results = HashSet::new();
                    results.insert(input_ind); // Zero matches

                    for end_pos in solve(input_chars, regex_ast, input_ind) {
                        results.insert(end_pos); // One match
                    }
                    results.into_iter().collect()
                }

                Repetition::Star => {
                    let mut results = HashSet::new();
                    let mut current_positions = vec![input_ind];
                    results.insert(input_ind); // Zero matches

                    loop {
                        let mut next_positions = HashSet::new();
                        let mut found_new = false;

                        for &pos in &current_positions {
                            for end_pos in solve(input_chars, regex_ast, pos) {
                                if !results.contains(&end_pos) {
                                    found_new = true;
                                    next_positions.insert(end_pos);
                                    results.insert(end_pos);
                                }
                            }
                        }

                        if !found_new {
                            break;
                        }
                        current_positions = next_positions.into_iter().collect();
                    }
                    results.into_iter().collect()
                }

                Repetition::Plus => {
                    let mut results = HashSet::new();
                    let mut current_positions: Vec<usize> =
                        solve(input_chars, regex_ast, input_ind);

                    // Add results from first mandatory match
                    for &pos in &current_positions {
                        results.insert(pos);
                    }

                    // Add results from additional matches (like Star)
                    loop {
                        let mut next_positions = HashSet::new();
                        let mut found_new = false;

                        for &pos in &current_positions {
                            for end_pos in solve(input_chars, regex_ast, pos) {
                                if !results.contains(&end_pos) {
                                    found_new = true;
                                    next_positions.insert(end_pos);
                                    results.insert(end_pos);
                                }
                            }
                        }

                        if !found_new {
                            break;
                        }
                        current_positions = next_positions.into_iter().collect();
                    }
                    results.into_iter().collect()
                }
            }
        }
    }
}



fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let ast = pattern_to_ast(pattern);
    let input_chars: Vec<char> = input_line.chars().collect();

    // Try matching from every starting position
    for start_pos in 0..=input_chars.len() {
        let end_positions = solve(&input_chars, &ast, start_pos);
       
        if !end_positions.is_empty() {
            return true;
        }
    }
    false
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
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
