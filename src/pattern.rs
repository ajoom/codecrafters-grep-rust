#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Repetition {
    None,
    Plus,  // +
    Star,  // *
}



#[derive(Debug, PartialEq)]
pub enum RegPattern {
    Digit(Repetition),                 // \d
    Word(Repetition),                  // \w
    PositiveGroup(String, Repetition), // [abc]
    NegativeGroup(String, Repetition), // [^abc]
    Literal(char, Repetition),
    StartOfLine,
    EndOfLine,
}
