
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Repetition {
    None,
    Plus,     // +
    Star,     // *
    Optional, // ?
}


#[derive(Debug, PartialEq, Clone)]
pub enum RegexAst {
    Concat(Vec<RegexAst>),             // sequence of nodes
    Alternate(Vec<RegexAst>),          // alternation a|b|c
    Repeat(Box<RegexAst>, Repetition), // repetition (*, +, ?)
    CaptureGroup(u32, Box<RegexAst>),  // (cat) the u32 represents the id of the group
    Digit,                             // \d
    Word,                              // \w
    PositiveGroup(String),             // [abc]
    NegativeGroup(String),             // [^abc]
    Literal(char),                     // 'a'
    Wildcard,                          // .
    StartOfLine,                       // ^
    EndOfLine,                         // $
}

/*
    regex = alternate
    alternate = concat ( | concat ) *
    concat = repeat +
    repeat = atom ('*' | '+')?
    atom = literal | group | class | anchor
*/
