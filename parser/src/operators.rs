use crate::functions::Function;
use lexer::token::LexerToken;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    LeftBracket(Bracket),
    Function(Function),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bracket {
    Absolute,
    Parenthesis,
}
impl<'a> TryFrom<LexerToken<'a>> for Operator {
    type Error = ();
    fn try_from(value: LexerToken<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            LexerToken::Plus => Self::Add,
            LexerToken::Minus => Self::Sub,
            LexerToken::Asterisk => Self::Mul,
            LexerToken::FowardSlash => Self::Div,
            LexerToken::Caret => Self::Pow,
            LexerToken::LeftParenthesis => Self::LeftBracket(Bracket::Parenthesis),
            LexerToken::Bar => Self::LeftBracket(Bracket::Absolute),
            _ => return Err(()),
        })
    }
}
