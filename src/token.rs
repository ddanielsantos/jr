#[derive(Debug)]
pub enum Token {
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    Quote,
    Comma,
    Semicolon,
    Dot,
    WhiteSpace,
    Colon,
    Minus,
    Number,
    String,
}
