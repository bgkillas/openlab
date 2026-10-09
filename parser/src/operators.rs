use crate::functions::Function;
use core::iter::Peekable;
use lexer::iter::LexerTokenIter;
use lexer::token::LexerToken;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Root,
    Factorial,
    SubFactorial,
    Negate,
    Mod,
    LeftBracket(Bracket),
    Function(Function),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bracket {
    Absolute,
    Parenthesis,
}
impl Operator {
    pub fn parse(token: LexerToken, iter: &mut Peekable<LexerTokenIter<'_>>) -> Option<Self> {
        Some(match token {
            LexerToken::Plus => Self::Add,
            LexerToken::Minus => Self::Sub,
            LexerToken::Asterisk if iter.next_if_eq(&LexerToken::Asterisk).is_some() => Self::Pow,
            LexerToken::Asterisk => Self::Mul,
            LexerToken::FowardSlash if iter.next_if_eq(&LexerToken::FowardSlash).is_some() => {
                Self::Root
            }
            LexerToken::FowardSlash => Self::Div,
            LexerToken::Caret => Self::Pow,
            LexerToken::Percent => Self::Mod,
            LexerToken::ExclamationMark => Self::Factorial,
            LexerToken::LeftParenthesis => Self::LeftBracket(Bracket::Parenthesis),
            LexerToken::Bar => Self::LeftBracket(Bracket::Absolute),
            _ => return None,
        })
    }
    pub fn inputs(self) -> u8 {
        match self {
            Self::Negate | Self::Factorial | Self::SubFactorial => 1,
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Pow | Self::Root | Self::Mod => 2,
            Self::Function(f) => f.inputs(),
            Self::LeftBracket(_) => {
                unreachable!()
            }
        }
    }
    pub fn inner_vars(self) -> u8 {
        if let Self::Function(fun) = self {
            fun.inner_vars()
        } else {
            0
        }
    }
    pub fn unary_right(self) -> bool {
        matches!(self, Self::Factorial)
    }
    pub fn get_unary_left(self) -> Option<Self> {
        Some(match self {
            Self::Add => Self::Add,
            Self::Sub => Self::Negate,
            Self::Factorial => Self::SubFactorial,
            _ => return None,
        })
    }
    pub fn is_unary(self) -> bool {
        matches!(self, Self::Negate | Self::SubFactorial | Self::Factorial)
    }
    pub fn unary_left(self) -> bool {
        match self {
            Self::Negate | Self::SubFactorial => true,
            Self::Factorial => false,
            _ => unreachable!(),
        }
    }
    pub fn precedence(self) -> u8 {
        match self {
            /*Self::Or => 0,
            Self::And => 1,
            Self::Equal
            | Self::NotEqual
            | Self::Greater
            | Self::Less
            | Self::LessEqual
            | Self::GreaterEqual => 2,*/
            //Self::Solve => 3,
            //Self::Convert => 4,
            Self::Add | Self::Sub => 5,
            Self::Mul | Self::Div => 6,
            Self::Negate => 7,
            Self::Pow | Self::Root => 8,
            Self::Mod => 9,
            Self::Factorial | Self::SubFactorial => 10,
            Self::LeftBracket(_) | Self::Function(_) => unreachable!(),
        }
    }
    pub fn left_associative(self) -> bool {
        match self {
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Mod => true,
            Self::Pow | Self::Root | Self::Negate | Self::Factorial | Self::SubFactorial => false,
            Self::LeftBracket(_) | Self::Function(_) => unreachable!(),
        }
    }
}
