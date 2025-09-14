#[derive(Debug, PartialEq)]
pub enum RegPattern {
    Digit,                 // \d
    Word,                  // \w
    PositiveGroup(String), // [abc]
    NegativeGroup(String), // [^abc]
    Literal(char),
    StartOfLine,
    EndOfLine,
}
