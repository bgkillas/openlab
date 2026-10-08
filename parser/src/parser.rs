use crate::functions::Function;
use core::iter::Peekable;
use lexer::iter::LexerTokenIter;
use lexer::token::LexerToken;
use number::float::complex::Complex;
pub type NumberInner = Complex<f64>;
pub type Number = number::number::Number<NumberInner>;
#[derive(Clone, Default)]
pub struct Expression {
    pub tokens: Vec<ExpressionToken>,
}
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Variable {
    Variable(u32),
    Function(u32),
    Global(u32),
    None,
}
#[derive(Clone)]
pub enum ExpressionToken {
    Number(Number),
    Variable(Variable),
    Function(Function),
    Skip(u32),
}
#[derive(Clone, Default)]
pub struct Block {
    pub blocks: Vec<BlockToken>,
}
#[derive(Clone)]
pub enum BlockToken {
    Expression(Expression),
    Assignment(Variable, Block),
    Return(Vec<Block>),
    PrintExpression(Expression),
    PrintAssignment(Variable, Block),
    PrintReturn(Vec<Block>),
    Last(Vec<Block>),
}
pub struct Parser<'a> {
    pub tokens: LexerTokenIter<'a>,
}
impl Block {
    pub fn parse(s: &str) -> Option<Block> {
        let mut tokens = LexerTokenIter::new(s).peekable();
        Self::parse_tokens(&mut tokens)
    }
    pub fn parse_tokens(tokens: &mut Peekable<LexerTokenIter>) -> Option<Block> {
        let mut block = Block::default();
        while let Some(token) = tokens.next() {
            match token {
                LexerToken::UnexpectedEnd => return None,
                LexerToken::Word("fn") => todo!(),
                LexerToken::Word("let") => todo!(),
                LexerToken::Word("mut") => todo!(),
                LexerToken::Word("return") => todo!(),
                LexerToken::LeftParenthesis => todo!(),
                _ => {
                    block.blocks.push(BlockToken::parse_expression(tokens)?);
                }
            }
        }
        Some(block)
    }
}
impl BlockToken {
    pub fn parse_expression(tokens: &mut Peekable<LexerTokenIter>) -> Option<Self> {
        todo!()
    }
}
