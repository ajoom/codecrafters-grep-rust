#[derive(Debug)]
pub enum RegPattern {
    Digit,                 // \d
    Word,                  // \w
    PositiveGroup(String), // [abc]
    NegativeGroup(String), // [^abc]
    Literal(char),
}
