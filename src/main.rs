use std::env;
use std::io;
use std::process;

use crate::pattern::RegPattern;
use crate::pattern::Repetition;
use crate::utils::match_pattern_with_char;
use crate::utils::patterns_to_vec;
mod pattern;
mod utils;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let patterns = patterns_to_vec(pattern);
    let input_chars: Vec<char> = input_line.chars().collect();
    let mut input_ind = 0;
    let mut next_char_must_match = false;

    eprintln!("Parsed patterns: {:?}", patterns);

    for pattern in &patterns {
        match pattern {
            RegPattern::StartOfLine => {
                next_char_must_match = true;
            }

            RegPattern::EndOfLine => {
                return input_ind == input_chars.len();
            }

            RegPattern::Digit(rep)
            | RegPattern::Word(rep)
            | RegPattern::Literal(_, rep)
            | RegPattern::PositiveGroup(_, rep)
            | RegPattern::NegativeGroup(_, rep) => {
                let mut matched = 0;

                // Match characters according to the pattern
                while input_ind < input_chars.len()
                    && match_pattern_with_char(pattern, input_chars[input_ind])
                {
                    input_ind += 1;
                    matched += 1;

                    // Stop early if repetition is None
                    if let Repetition::None = rep {
                        break;
                    }
                }

                // Validate match counts based on repetition
                match rep {
                    Repetition::None => {
                        if matched != 1 {
                            return false;
                        }
                    }
                    Repetition::Plus => {
                        if matched < 1 {
                            return false;
                        }
                    }
                    Repetition::Star => {
                        // zero or more, always ok
                    }
                }

                if next_char_must_match && matched == 0 {
                    return false;
                }

                next_char_must_match = false;
            }
        }
    }

    true
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
