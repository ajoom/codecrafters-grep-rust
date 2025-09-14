use std::cmp::min;
use std::env;
use std::io;
use std::process;

use crate::pattern::RegexAst;
use crate::utils::match_pattern_with_char;
use crate::utils::pattern_to_ast;
mod old_main;
mod pattern;
mod utils;



fn solve(input_chars: &[char], node: &RegexAst, input_ind: usize) -> bool {
    
    if input_ind == input_chars.len() {
        return *node == RegexAst::EndOfLine;
    }

    match node {
        RegexAst::Digit 
        | RegexAst::Word
        | RegexAst::PositiveGroup(_) 
        | RegexAst::NegativeGroup(_)
        | RegexAst::Literal(_)
        | RegexAst::Wildcard => {
            match_pattern_with_char(node, input_chars[input_ind])
        }


        RegexAst::StartOfLine => panic!("^ should not be in input_chars"),
        RegexAst::EndOfLine =>  panic!("$ should not be in input_chars"),


        
        RegexAst::Concat(regex_asts) => todo!(),
        RegexAst::Alternate(regex_asts) => todo!(),
        RegexAst::Repeat(regex_ast, repetition) => todo!(),

    }
}


fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let ast = pattern_to_ast(pattern);
    eprintln!("{:?}", ast);
    let input_chars: Vec<char> = input_line.chars().collect();

    // TODO handle ^ and $ here, (try each pair of [st, en] ?)
    // input_chars should not contain any of ^ or $

    solve(&input_chars, &ast, 0)
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
