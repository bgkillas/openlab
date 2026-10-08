use crate::functions::Function;
use core::iter::Peekable;
use lexer::iter::LexerTokenIter;
use lexer::token::LexerToken;
use number::float::complex::Complex;
use number::traits::base::BaseImpl as _;
pub type NumberInner = Complex<f64>;
pub type Number = number::number::Number<NumberInner>;
#[derive(Clone, Default, Debug)]
pub struct Expression {
    pub tokens: Box<[ExpressionToken]>,
}
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Variable {
    Variable(u32),
    Function(u32),
    Global(u32),
    None,
}
#[derive(Clone, Debug)]
pub enum ExpressionToken {
    Number(Number),
    Variable(Variable),
    Function(Function),
    Skip(u32),
}
#[derive(Clone, Default, Debug)]
pub struct Block<'a> {
    pub blocks: Box<[BlockToken<'a>]>,
}
#[derive(Clone, Debug)]
pub enum BlockToken<'a> {
    Function(&'a str, Block<'a>),
    Expression(Expression),
    Assignment(Variable, Box<[Block<'a>]>),
    Return(Box<[Block<'a>]>),
    PrintExpression(Expression),
    PrintAssignment(Variable, Box<[Block<'a>]>),
    PrintReturn(Box<[Block<'a>]>),
    Last(Box<[Block<'a>]>),
}
impl<'a> Block<'a> {
    pub fn parse(s: &'a str) -> Option<Block<'a>> {
        let mut tokens = LexerTokenIter::new(s).peekable();
        Self::parse_tokens(&mut tokens)
    }
    pub fn parse_tokens(tokens: &mut Peekable<LexerTokenIter<'a>>) -> Option<Block<'a>> {
        let mut blocks = Vec::new();
        while let Some(token) = tokens.peek() {
            let part = match token {
                LexerToken::UnexpectedEnd => return None,
                LexerToken::Word("fn") => todo!(),
                LexerToken::Word("let") => todo!(),
                LexerToken::Word("mut") => todo!(),
                LexerToken::Word("return") => todo!(),
                LexerToken::LeftParenthesis => todo!(),
                _ => BlockToken::parse_expression(tokens)?,
            };
            blocks.push(part);
        }
        Some(Self {
            blocks: blocks.into_boxed_slice(),
        })
    }
}
impl<'a> BlockToken<'a> {
    pub fn parse_expression(tokens: &mut Peekable<LexerTokenIter<'a>>) -> Option<Self> {
        let mut expr = Vec::new();
        let mut print = false;
        while let Some(token) = tokens.next() {
            let part = match token {
                LexerToken::UnexpectedEnd => return None,
                LexerToken::BackSlash if tokens.peek() == Some(&LexerToken::NewLine) => continue,
                LexerToken::Numeric(base, whole, part) => ExpressionToken::Number(Number::Complex(
                    NumberInner::parse_number(base.unwrap_or(10), whole, part.unwrap_or(0)),
                )),
                LexerToken::SemiColon => {
                    print = true;
                    continue;
                }
                _ => todo!(),
            };
            expr.push(part);
        }
        let expression = Expression {
            tokens: expr.into_boxed_slice(),
        };
        Some(if print {
            Self::PrintExpression(expression)
        } else {
            Self::Expression(expression)
        })
    }
}
