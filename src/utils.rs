use core::panic;

use crate::pattern::{RegPattern, Repetition};

pub fn patterns_to_vec(pattern: &str) -> Vec<RegPattern> {
    let mut pattern_ind = 0;
    let mut result = vec![];

    while pattern_ind < pattern.len() {
        if (pattern_ind + 1) < pattern.len() {
            // look for digit pattern
            if &pattern[pattern_ind..pattern_ind + 2] == r"\d" {
                let repetition = get_repition_type(pattern, pattern_ind + 1);

                pattern_ind += 2;
                if repetition != Repetition::None {
                    pattern_ind += 1;
                }

                result.push(RegPattern::Digit(repetition));
                continue;
            }

            // look for word pattern
            if &pattern[pattern_ind..pattern_ind + 2] == r"\w" {
                let repetition = get_repition_type(pattern, pattern_ind + 1);

                pattern_ind += 2;
                if repetition != Repetition::None {
                    pattern_ind += 1;
                }

                result.push(RegPattern::Word(repetition));
                continue;
            }

            // look for positive/ negative group pattern
            if pattern.chars().nth(pattern_ind).unwrap() == '[' {
                let closing_bracket_ind = pattern[pattern_ind..].find(']');

                if closing_bracket_ind.is_none() {
                    panic!("Unhandled pattern: {pattern}");
                }

                let closing_bracket_ind = closing_bracket_ind.unwrap() + pattern_ind;

                let repition = get_repition_type(pattern, closing_bracket_ind - 1);

                if pattern.chars().nth(pattern_ind + 1).unwrap() == '^' {
                    result.push(RegPattern::NegativeGroup(
                        pattern[pattern_ind + 2..closing_bracket_ind].to_string(),
                        repition,
                    ));
                } else {
                    result.push(RegPattern::PositiveGroup(
                        pattern[pattern_ind + 1..closing_bracket_ind].to_string(),
                        repition,
                    ));
                }

                pattern_ind = closing_bracket_ind + 1;
                if repition != Repetition::None {
                    pattern_ind += 1;
                }
                continue;
            }
        }

        if pattern.chars().nth(pattern_ind).unwrap() == '^' {
            result.push(RegPattern::StartOfLine);
            pattern_ind += 1;
            continue;
        }

        if pattern.chars().nth(pattern_ind).unwrap() == '$' {
            result.push(RegPattern::EndOfLine);
            pattern_ind += 1;
            continue;
        }

        // literal pattern :)
        let repition = get_repition_type(pattern, pattern_ind);

        result.push(match pattern.chars().nth(pattern_ind).unwrap() {
            '.' => RegPattern::Wildcard(repition),
            _ => RegPattern::Literal(pattern.chars().nth(pattern_ind).unwrap(), repition),
        });

        pattern_ind += 1;
        if repition != Repetition::None {
            pattern_ind += 1;
        }
    }

    result
}

pub fn match_pattern_with_char(pattern: &RegPattern, c: char) -> bool {
    match pattern {
        RegPattern::Digit(_) => c.is_ascii_digit(),

        RegPattern::Word(_) => c.is_ascii_alphanumeric() || c == '_',

        RegPattern::PositiveGroup(group, _) => group.chars().any(|gc| gc == c),

        RegPattern::NegativeGroup(group, _) => group.chars().all(|gc| gc != c),

        RegPattern::Literal(l, _) => c == *l,

        RegPattern::Wildcard(_) => true,

        _ => panic!("should not be matched with char"),
    }
}

fn get_repition_type(pattern: &str, last_index_in_pattern: usize) -> Repetition {
    if last_index_in_pattern + 1 >= pattern.len() {
        return Repetition::None;
    }

    match pattern.chars().nth(last_index_in_pattern + 1).unwrap() {
        '?' => Repetition::Star,
        '+' => Repetition::Plus,
        _ => Repetition::None,
    }
}
