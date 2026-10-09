use crate::functions::Function;
use crate::operators::Operator;
use core::iter::Peekable;
use core::str::FromStr as _;
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
    pub fn parse(s: &'a str) -> Option<Self> {
        let mut tokens = LexerTokenIter::new(s).peekable();
        Self::parse_tokens(&mut tokens)
    }
    pub fn parse_tokens(tokens: &mut Peekable<LexerTokenIter<'a>>) -> Option<Self> {
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
#[derive(Default)]
struct State {
    expression: Vec<ExpressionToken>,
    operator_stack: Vec<Operator>,
}
impl State {
    fn push_operator(&mut self, operator: Operator) {
        self.operator_stack.push(operator);
        todo!()
    }
    fn push_number(&mut self, number: NumberInner) {
        self.expression
            .push(ExpressionToken::Number(Number::Complex(number)));
    }
}
impl<'a> BlockToken<'a> {
    pub fn parse_expression(tokens: &mut Peekable<LexerTokenIter<'a>>) -> Option<Self> {
        let mut state = State::default();
        let mut print = false;
        while let Some(token) = tokens.next() {
            match token {
                LexerToken::UnexpectedEnd => return None,
                LexerToken::BackSlash if tokens.next_if_eq(&LexerToken::NewLine).is_some() => {}
                LexerToken::Numeric(base, whole, part) => {
                    let number =
                        NumberInner::parse_number(base.unwrap_or(10), whole, part.unwrap_or(0));
                    state.push_number(number);
                }
                LexerToken::SemiColon => {
                    print = true;
                }
                LexerToken::Word(str) if let Ok(fun) = Function::from_str(str) => {
                    state.push_operator(Operator::Function(fun));
                }
                t if let Ok(op) = Operator::try_from(t) => {
                    state.push_operator(op);
                }
                _ => todo!(),
            }
        }
        let expression = Expression {
            tokens: state.expression.into_boxed_slice(),
        };
        Some(if print {
            Self::PrintExpression(expression)
        } else {
            Self::Expression(expression)
        })
    }
}
