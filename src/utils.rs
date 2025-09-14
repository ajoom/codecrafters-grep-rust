use core::panic;

use crate::pattern::{RegexAst, Repetition};





pub fn pattern_to_ast(pattern: &str) -> RegexAst {
    let mut pattern_ind = 0;

    parse_alternation(pattern, &mut pattern_ind)
}


fn parse_alternation(pattern: &str, pattern_ind: &mut usize) -> RegexAst {
    let mut branches = vec![parse_concatination(pattern, pattern_ind)];
    
    while *pattern_ind < pattern.len() && pattern.chars().nth(*pattern_ind).unwrap() == '|' {
        *pattern_ind += 1;
        branches.push(parse_concatination(pattern, pattern_ind));
    }
    
    // Only create Alternate if there are multiple branches
    if branches.len() == 1 {
        branches.into_iter().next().unwrap()
    } else {
        RegexAst::Alternate(branches)
    }
}

fn parse_concatination(pattern: &str, pattern_ind: &mut usize) -> RegexAst {
    let mut parts = vec![];
    
    while *pattern_ind < pattern.len() {
        let c = pattern.chars().nth(*pattern_ind).unwrap();
        if c == ')' || c == '|' {
            break;
        }
        parts.push(parse_repeat(pattern, pattern_ind));
    }
    
    // Only create Concat if there are multiple parts
    if parts.len() == 1 {
        parts.into_iter().next().unwrap()
    } else {
        RegexAst::Concat(parts)
    }
}



fn parse_repeat(pattern: &str, pattern_ind: &mut usize) -> RegexAst {
    let node = parse_atom(pattern, pattern_ind);
    let rep = get_repition_type(pattern, pattern_ind);
    if rep == Repetition::None {
        node
    } else {
        RegexAst::Repeat(Box::new(node), rep)
    }
}

fn parse_atom(pattern: &str, pattern_ind: &mut usize) -> RegexAst {
    match pattern.chars().nth(*pattern_ind).unwrap() {
        '^' => {
            *pattern_ind += 1;
            RegexAst::StartOfLine
        },

        '$' => {
            *pattern_ind += 1;
            RegexAst::EndOfLine
        }, 

        '.' => {
            *pattern_ind += 1;
            RegexAst::Wildcard
        },


        '(' => {
            *pattern_ind += 1;
            let node = parse_alternation(pattern, pattern_ind);

            if *pattern_ind >= pattern.len() || pattern.chars().nth(*pattern_ind).unwrap() != ')' {
                panic!("involid pattern, ( is not closed")
            }

            *pattern_ind += 1;

            node
        }


        '[' => {
            *pattern_ind += 1;
            let mut negative_group = false;

            if pattern.chars().nth(*pattern_ind).unwrap() == '^' {
                *pattern_ind += 1;
                negative_group = true;
            }

            let closing_bracket_ind = pattern[*pattern_ind..].find(']');

            if closing_bracket_ind.is_none() {
                panic!("Unhandled pattern: {pattern}");
            }

            let closing_bracket_ind = closing_bracket_ind.unwrap() + *pattern_ind;
            let group = pattern[*pattern_ind..closing_bracket_ind].to_string();
            
            *pattern_ind = closing_bracket_ind + 1;

            match negative_group {
                true => RegexAst::NegativeGroup(group),
                false => RegexAst::PositiveGroup(group),
            }
        }

        '\\' => {
            *pattern_ind += 1;
            match pattern.chars().nth(*pattern_ind).unwrap() {
                'w' => {
                    *pattern_ind += 1;
                    RegexAst::Word
                }
                'd' => {
                    *pattern_ind += 1;
                    RegexAst::Digit
                },
                _ => panic!("non valid regex pattern")
            }
        }, 

        literal=> {
            *pattern_ind += 1;
            RegexAst::Literal(literal)
        }
    }
}




fn get_repition_type(pattern: &str, last_index_in_pattern: &mut usize) -> Repetition {
    if *last_index_in_pattern >= pattern.len() {
        return Repetition::None;
    }

    match pattern.chars().nth(*last_index_in_pattern).unwrap() {
        '*' => { *last_index_in_pattern += 1; Repetition::Star }
        '+' => { *last_index_in_pattern += 1; Repetition::Plus }
        '?' => { *last_index_in_pattern += 1; Repetition::Optional }
        _ => Repetition::None,
    }
}
