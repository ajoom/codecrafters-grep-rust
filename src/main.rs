use std::env;
use std::io;
use std::process;

use crate::pattern::RegPattern;
use crate::pattern::Repetition;
use crate::utils::match_pattern_with_char;
use crate::utils::patterns_to_vec;
mod pattern;
mod utils;

fn solve(input_chars: &Vec<char>, patterns: &Vec<RegPattern>, input_ind: usize, pattern_ind: usize, next_char_must_match: bool, last_matched_on_pattern: bool) -> bool {
    if pattern_ind == patterns.len() {
        return true;
    }

    if input_ind == input_chars.len() {
        return false;
    }

    let pattern =  &patterns[pattern_ind];

    match pattern {
        RegPattern::StartOfLine => solve(input_chars, patterns, input_ind, pattern_ind + 1, true, false),

        RegPattern::EndOfLine => input_ind == input_chars.len(),


         RegPattern::Digit(rep)
            | RegPattern::Word(rep)
            | RegPattern::Literal(_, rep)
            | RegPattern::PositiveGroup(_, rep)
            | RegPattern::NegativeGroup(_, rep) => {
                
                let matches =  input_ind < input_chars.len() && match_pattern_with_char(&pattern, input_chars[input_ind]);
                
                if next_char_must_match && !matches {
                    return false;
                }

                match rep {
                    Repetition::None => {
                        if !matches {
                            return false;
                        }
                        
                        solve(input_chars, patterns, input_ind + 1, pattern_ind + 1, false, false)
                    },
                    

                    Repetition::Plus => {
                        if !matches {
                            if last_matched_on_pattern {
                                return solve(input_chars, patterns, input_ind, pattern_ind + 1, false, false);
                            } else {
                                return false;
                            }
                        }

                        solve(input_chars, patterns, input_ind + 1, pattern_ind + 1, false, false)
                        | solve(input_chars, patterns, input_ind + 1, pattern_ind, false, false)
                    },
                    
                    Repetition::Star => todo!(),
                }
        }
    }
}




fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let patterns = patterns_to_vec(pattern);
    let input_chars: Vec<char> = input_line.chars().collect();

    eprintln!("Parsed patterns: {:?}", patterns);

    solve(&input_chars, &patterns, 0, 0, false, false)
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
