use std::cmp::min;
use std::env;
use std::io;
use std::process;

use crate::pattern::RegexAst;
use crate::pattern::Repetition;
use crate::utils::pattern_to_ast;
mod old_main;
mod pattern;
mod utils;



const INF: usize = 1000_000_000_usize;

// returns the smallest length of input with solution, or INF if solution was not found
fn solve(input_chars: &[char], node: &RegexAst, input_ind: usize, next_char_must_match: bool, last_matched_on_pattern: bool) -> usize {
    eprintln!("solve: input_ind={}, node={:?}, must_match={}", input_ind, node, next_char_must_match);
    
    if input_ind >= input_chars.len() {
        let result = match node {
            RegexAst::EndOfLine => input_chars.len(),
            RegexAst::Repeat(_, Repetition::Star) => input_chars.len(), 
            RegexAst::Repeat(_, Repetition::Optional) => input_chars.len(), 
            RegexAst::Concat(nodes) if nodes.is_empty() => input_chars.len(), 
            _ => INF
        };
        eprintln!("  -> end of input, returning {}", result);
        return result;
    }

    let result = match node {
        RegexAst::Literal(c) => {
            let matches = input_chars[input_ind] == *c;
            eprintln!("  -> Literal '{}' vs '{}': {}", c, input_chars[input_ind], matches);
            match matches {
                true => input_ind + 1,
                false => {
                    if next_char_must_match {
                        INF
                    } else {
                        solve(input_chars, node, input_ind + 1, false, false)
                    }
                }
            }
        }

        RegexAst::Concat(nodes) => {
            eprintln!("  -> Concat with {} nodes", nodes.len());
            if nodes.is_empty() {
                return input_ind;
            }
            
            if next_char_must_match {
                eprintln!("    -> Must match sequence at position {}", input_ind);
                let mut current_pos = input_ind;
                for (i, node) in nodes.iter().enumerate() {
                    eprintln!("      -> Matching node {} at position {}", i, current_pos);
                    let result = solve(input_chars, node, current_pos, true, false);
                    if result == INF {
                        eprintln!("      -> Node {} failed", i);
                        return INF;
                    }
                    current_pos = result;
                    eprintln!("      -> Node {} succeeded, now at position {}", i, current_pos);
                }
                eprintln!("    -> Sequence matched, ending at {}", current_pos);
                current_pos
            } else {
                eprintln!("    -> Trying sequence at different positions starting from {}", input_ind);
                for start_pos in input_ind..=input_chars.len() {
                    eprintln!("      -> Trying at position {}", start_pos);
                    let mut current_pos = start_pos;
                    let mut sequence_matches = true;
                    
                    for (i, node) in nodes.iter().enumerate() {
                        let result = solve(input_chars, node, current_pos, true, false);
                        if result == INF {
                            eprintln!("        -> Node {} failed at pos {}", i, current_pos);
                            sequence_matches = false;
                            break;
                        }
                        current_pos = result;
                        eprintln!("        -> Node {} succeeded, now at {}", i, current_pos);
                    }
                    
                    if sequence_matches {
                        eprintln!("      -> Sequence found! Ending at {}", current_pos);
                        return current_pos;
                    }
                }
                eprintln!("    -> No sequence found");
                INF
            }
        }

        // ... other cases remain the same but let me add minimal ones for testing
        RegexAst::Digit => {
            let matches = input_chars[input_ind].is_ascii_digit();
            match matches {
                true => input_ind + 1,
                false => {
                    if next_char_must_match {
                        INF
                    } else {
                        solve(input_chars, node, input_ind + 1, false, false)
                    }
                }
            }
        }

        RegexAst::Word => {
            let matches = input_chars[input_ind].is_ascii_alphanumeric() || input_chars[input_ind] == '_';
            match matches {
                true => input_ind + 1,
                false => {
                    if next_char_must_match {
                        INF
                    } else {
                        solve(input_chars, node, input_ind + 1, false, false)
                    }
                }
            }
        }

        RegexAst::Wildcard => {
            if next_char_must_match {
                input_ind + 1
            } else {
                min(input_ind + 1, solve(input_chars, node, input_ind + 1, false, false))
            }
        }

        RegexAst::PositiveGroup(group) => {
            let matches = group.contains(input_chars[input_ind]);
            match matches {
                true => input_ind + 1,
                false => {
                    if next_char_must_match {
                        INF
                    } else {
                        solve(input_chars, node, input_ind + 1, false, false)
                    }
                }
            }
        }

        RegexAst::NegativeGroup(group) => {
            let matches = !group.contains(input_chars[input_ind]);
            match matches {
                true => input_ind + 1,
                false => {
                    if next_char_must_match {
                        INF
                    } else {
                        solve(input_chars, node, input_ind + 1, false, false)
                    }
                }
            }
        }

        RegexAst::StartOfLine => {
            let matches = input_ind == 0;
            match matches {
                true => input_ind,
                false => {
                    if next_char_must_match {
                        INF
                    } else {
                        solve(input_chars, node, input_ind + 1, false, false)
                    }
                }
            }
        }

        RegexAst::EndOfLine => {
            let matches = input_ind == input_chars.len();
            match matches {
                true => input_ind,
                false => {
                    if next_char_must_match {
                        INF
                    } else {
                        solve(input_chars, node, input_ind + 1, false, false)
                    }
                }
            }
        }

        RegexAst::Alternate(nodes) => {
            let mut min_index = INF;
            
            for sub in nodes {
                min_index = min(min_index, solve(input_chars, sub, input_ind, next_char_must_match, false));
            }
            
            min_index
        }

        RegexAst::Repeat(sub, rep) => match rep {
            Repetition::None => solve(input_chars, sub, input_ind, next_char_must_match, last_matched_on_pattern),
            _ => INF // TODO: implement other repetitions
        },
    };
    
    eprintln!("  -> returning {}", result);
    result
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let ast = pattern_to_ast(pattern);
    eprintln!("{:?}", ast);
    let input_chars: Vec<char> = input_line.chars().collect();
    eprintln!("Input: {:?}", input_chars);
    let result = solve(&input_chars, &ast, 0, false, false);
    eprintln!("Final result: {}", result);
    result != INF
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
