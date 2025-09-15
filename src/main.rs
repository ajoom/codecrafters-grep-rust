use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::process;
use std::vec;

use crate::pattern::RegexAst;
use crate::pattern::Repetition;
use crate::utils::match_pattern_with_char;
use crate::utils::pattern_to_ast;
mod old_main;
mod pattern;
mod utils;


// Returns all possible end positions after matching this node starting from input_ind
fn solve(input_chars: &[char], node: &RegexAst, input_ind: usize, captures: &mut HashMap<u32, String>) -> Vec<usize> {
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


        RegexAst::Backreference(group_id) => {
            let captured_group_text = captures.get(group_id);

            if let Some(captured_text) = captured_group_text {
                let captured_group_text: Vec<char> = captured_text.chars().collect();

                let remaining_input_len = input_chars.len() - input_ind;
                if remaining_input_len < captured_group_text.len() {
                    return vec![];
                }

                // now we can do the comparsion ;) 
                if input_chars[input_ind..input_ind + captured_group_text.len()] == captured_group_text[..] {
                    return vec![input_ind + captured_group_text.len()];
                }
            }

            vec![]
        }


        RegexAst::CaptureGroup(group_id, ast) => {
            let end_positions = solve(input_chars, ast, input_ind, captures);
            
            if !end_positions.is_empty() {
                // Only capture if there was at least one match
                let last_end = *end_positions.iter().max().unwrap();
                let matched_text: String = input_chars[input_ind..last_end].iter().collect();
                captures.insert(*group_id, matched_text);
            }
            
            end_positions
        }


        RegexAst::Alternate(regex_asts) => {
            let mut all_results = HashSet::new();
            
            for option in regex_asts {
                // Save current capture state for backtracking
                let saved_captures = captures.clone();
                
                let results = solve(input_chars, option, input_ind, captures);
                
                if results.is_empty() {
                    // This alternative failed, restore captures
                    *captures = saved_captures;
                } else {
                    // This alternative succeeded, keep the captures and add results
                    for end_pos in results {
                        all_results.insert(end_pos);
                    }
                    // For now, we'll take the first successful alternative
                    // In a more sophisticated implementation, we'd try all alternatives
                    break;
                }
            }
            
            all_results.into_iter().collect()
        }

        RegexAst::Concat(regex_asts) => {
            let mut current_positions = vec![input_ind];

            for ast in regex_asts {
                let mut next_positions = HashSet::new();
                for &pos in &current_positions {
                    for end_pos in solve(input_chars, ast, pos, captures) {
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



        RegexAst::Repeat(regex_ast, repetition) => {
            match repetition {
                Repetition::None => solve(input_chars, regex_ast, input_ind, captures),

                Repetition::Optional => {
                    let mut results = HashSet::new();
                    results.insert(input_ind); // Zero matches

                    for end_pos in solve(input_chars, regex_ast, input_ind, captures) {
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
                            for end_pos in solve(input_chars, regex_ast, pos, captures) {
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
                        solve(input_chars, regex_ast, input_ind, captures);

                    // Add results from first mandatory match
                    for &pos in &current_positions {
                        results.insert(pos);
                    }

                    // Add results from additional matches (like Star)
                    loop {
                        let mut next_positions = HashSet::new();
                        let mut found_new = false;

                        for &pos in &current_positions {
                            for end_pos in solve(input_chars, regex_ast, pos, captures) {
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
    let input_chars: Vec<char> = input_line.trim_end().chars().collect();

    eprintln!("{:?}", ast);
    
    // Try matching from every starting position
    for start_pos in 0..=input_chars.len() {
        let mut captures = HashMap::new();
        let end_positions = solve(&input_chars, &ast, start_pos, &mut captures);
       
        if !end_positions.is_empty() {
            return true;
        }
    }
    false
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
// Or: your_program.sh -E <pattern> <filename>
fn main() {
    eprintln!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("Usage: {} -E <pattern> [filename]", args[0]);
        process::exit(1);
    }
    
    if args[1] != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = &args[2];
    let mut found_match = false;

    if args.len() == 4 {
        // File mode: read from file
        let filename = &args[3];
        
        match fs::read_to_string(filename) {
            Ok(file_contents) => {
                // Process the file content (single line for this stage)
                let line = file_contents.trim_end();
                if match_pattern(line, pattern) {
                    println!("{}", line);
                    found_match = true;
                }
            }
            Err(err) => {
                eprintln!("Error reading file {}: {}", filename, err);
                process::exit(1);
            }
        }
    } else {
        // Stdin mode: read from standard input
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        
        if match_pattern(&input_line, pattern) {
            found_match = true;
        }
    }

    if found_match {
        process::exit(0);
    } else {
        process::exit(1);
    }
}