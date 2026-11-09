use crate::functions::Function;
use crate::parser::State;
use core::ops::Range;
use itertools::PeekNth;
use lexer::iter::LexerIter;
use lexer::token::LexerToken;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    MulInner,
    Div,
    Pow,
    Root,
    Factorial,
    SubFactorial,
    Negate,
    Mod,
    Identity,
    Transpose,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Eq,
    NotEq,
    And,
    Or,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperatorToken {
    Operator(Operator),
    ComponentWise(Operator),
    LeftBracket(Bracket),
    Function(Function),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bracket {
    Absolute,
    Parenthesis,
    Square,
}
impl OperatorToken {
    pub(crate) fn parse(
        state: &State,
        token: LexerToken,
        iter: &mut PeekNth<LexerIter<'_>>,
    ) -> Option<Self> {
        Some(if state.input_left {
            match token {
                LexerToken::LeftParenthesis => Self::LeftBracket(Bracket::Parenthesis),
                LexerToken::Bar => Self::LeftBracket(Bracket::Absolute),
                LexerToken::Colon => {
                    let ret = Self::ComponentWise(Operator::parse(state, iter.peek()?.1, iter, 1)?);
                    iter.next();
                    ret
                }
                _ => Self::Operator(Operator::parse(state, token, iter, 0)?),
            }
        } else if token == LexerToken::Colon {
            let ret = Self::Operator(Operator::parse(state, iter.peek()?.1, iter, 1)?);
            iter.next();
            ret
        } else {
            Self::Operator(Operator::parse(state, token, iter, 0)?)
        })
    }
    pub fn input_range<F>(self, custom: F) -> Range<u32>
    where
        F: FnOnce(u32) -> Range<u32>,
    {
        match self {
            Self::Operator(o) | Self::ComponentWise(o) => o.input_range(),
            Self::Function(f) => f.input_range(custom),
            Self::LeftBracket(_) => {
                unreachable!()
            }
        }
    }
    pub fn inputs<F>(self, custom: F) -> u32
    where
        F: FnOnce(u32) -> u32,
    {
        match self {
            Self::Operator(o) | Self::ComponentWise(o) => o.inputs(),
            Self::Function(f) => f.inputs(custom),
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
    pub fn is_unary(self) -> bool {
        match self {
            Self::Operator(o) | Self::ComponentWise(o) => o.is_unary(),
            _ => false,
        }
    }
    pub fn unary_left(self) -> bool {
        match self {
            Self::Operator(o) | Self::ComponentWise(o) => o.unary_left(),
            _ => unreachable!(),
        }
    }
    pub fn precedence(self) -> u8 {
        match self {
            Self::Operator(o) | Self::ComponentWise(o) => o.precedence(),
            Self::LeftBracket(_) | Self::Function(_) => unreachable!(),
        }
    }
    pub fn left_associative(self) -> bool {
        match self {
            Self::Operator(o) | Self::ComponentWise(o) => o.left_associative(),
            Self::LeftBracket(_) | Self::Function(_) => unreachable!(),
        }
    }
}
impl Operator {
    pub(crate) fn parse(
        state: &State,
        token: LexerToken,
        iter: &mut PeekNth<LexerIter<'_>>,
        i: usize,
    ) -> Option<Self> {
        Some(if state.input_left {
            match token {
                LexerToken::Plus => Self::Add,
                LexerToken::Minus => Self::Sub,
                LexerToken::Asterisk
                    if let Some(&(_, t)) = iter.peek_nth(i)
                        && t == LexerToken::Asterisk =>
                {
                    iter.next();
                    Self::Pow
                }
                LexerToken::SingleQuote => Self::Transpose,
                LexerToken::Asterisk => Self::Mul,
                LexerToken::FowardSlash
                    if let Some(&(_, t)) = iter.peek_nth(i)
                        && t == LexerToken::FowardSlash =>
                {
                    iter.next();
                    Self::Root
                }
                LexerToken::FowardSlash => Self::Div,
                LexerToken::Caret => Self::Pow,
                LexerToken::Percent => Self::Mod,
                LexerToken::Word("and") => Self::And,
                LexerToken::Word("or") => Self::Or,
                LexerToken::Equal
                    if let Some(&(_, t)) = iter.peek_nth(i)
                        && t == LexerToken::Equal =>
                {
                    iter.next();
                    Self::Eq
                }
                LexerToken::ExclamationMark
                    if let Some(&(_, t)) = iter.peek_nth(i)
                        && t == LexerToken::Equal =>
                {
                    iter.next();
                    Self::NotEq
                }
                LexerToken::ExclamationMark => Self::Factorial,
                LexerToken::LeftAngleBracket
                    if let Some(&(_, t)) = iter.peek_nth(i)
                        && t == LexerToken::Equal =>
                {
                    iter.next();
                    Self::LessEq
                }
                LexerToken::RightAngleBracket
                    if let Some(&(_, t)) = iter.peek_nth(i)
                        && t == LexerToken::Equal =>
                {
                    iter.next();
                    Self::GreaterEq
                }
                LexerToken::LeftAngleBracket => Self::Less,
                LexerToken::RightAngleBracket => Self::Greater,
                _ => return None,
            }
        } else {
            match token {
                LexerToken::Plus => Self::Identity,
                LexerToken::Minus => Self::Negate,
                LexerToken::ExclamationMark => Self::SubFactorial,
                _ => return None,
            }
        })
    }
    pub fn input_range(self) -> Range<u32> {
        match self {
            Self::Negate
            | Self::Factorial
            | Self::SubFactorial
            | Self::Identity
            | Self::Transpose => 1..2,
            Self::Add
            | Self::Sub
            | Self::Mul
            | Self::MulInner
            | Self::Div
            | Self::Pow
            | Self::Root
            | Self::Mod
            | Self::Less
            | Self::LessEq
            | Self::Greater
            | Self::GreaterEq
            | Self::Eq
            | Self::NotEq
            | Self::And
            | Self::Or => 2..3,
        }
    }
    pub fn inputs(self) -> u32 {
        match self {
            Self::Negate
            | Self::Factorial
            | Self::SubFactorial
            | Self::Identity
            | Self::Transpose => 1,
            Self::Add
            | Self::Sub
            | Self::Mul
            | Self::MulInner
            | Self::Div
            | Self::Pow
            | Self::Root
            | Self::Mod
            | Self::Less
            | Self::LessEq
            | Self::Greater
            | Self::GreaterEq
            | Self::Eq
            | Self::NotEq
            | Self::And
            | Self::Or => 2,
        }
    }
    pub fn inner_vars(self) -> u8 {
        0
    }
    pub fn is_unary(self) -> bool {
        matches!(
            self,
            Self::Negate | Self::SubFactorial | Self::Factorial | Self::Transpose
        )
    }
    pub fn unary_left(self) -> bool {
        match self {
            Self::Negate | Self::SubFactorial => true,
            Self::Factorial | Self::Transpose => false,
            _ => unreachable!(),
        }
    }
    pub fn precedence(self) -> u8 {
        match self {
            Self::Or => 0,
            Self::And => 1,
            Self::Less
            | Self::LessEq
            | Self::Greater
            | Self::GreaterEq
            | Self::Eq
            | Self::NotEq => 2,
            //Self::Solve => 3,
            //Self::Convert => 4,
            Self::Add | Self::Sub => 5,
            Self::Mul | Self::Div => 6,
            Self::Negate | Self::Identity | Self::MulInner => 7,
            Self::Pow | Self::Root => 8,
            Self::Mod => 9,
            Self::Factorial | Self::SubFactorial | Self::Transpose => 10,
        }
    }
    pub fn left_associative(self) -> bool {
        match self {
            Self::Add
            | Self::Sub
            | Self::Mul
            | Self::MulInner
            | Self::Div
            | Self::Mod
            | Self::And
            | Self::Or => true,
            Self::Pow
            | Self::Root
            | Self::Negate
            | Self::Identity
            | Self::Factorial
            | Self::SubFactorial
            | Self::Transpose
            | Self::Less
            | Self::LessEq
            | Self::Greater
            | Self::GreaterEq
            | Self::Eq
            | Self::NotEq => false,
        }
    }
}
