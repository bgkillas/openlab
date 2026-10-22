use crate::functions::Function;
use crate::operators::{Bracket, Operator, OperatorToken};
use crate::options::ParserOptions;
use crate::units::{Dimension, UNIT_COUNT, UnitType};
use core::str::FromStr as _;
use itertools::{PeekNth, peek_nth};
use lexer::iter::LexerIter;
use lexer::token::LexerToken;
use number::float::complex::Complex;
use number::traits::number::NumberImpl as _;
use number::units::NumberUnits;
pub type Float = f64;
pub type NumberBase = Complex<Float>;
pub type NumberInner = NumberUnits<NumberBase, UnitType, UNIT_COUNT>;
pub type Number = number::number::Number<NumberInner>;
#[derive(Clone, Default, Debug)]
pub struct Expression<'a> {
    pub tokens: Box<[ExpressionToken<'a>]>,
}
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Variable {
    Variable(u32),
    Function(u32),
    Global(u32),
    None,
}
#[derive(Clone, Debug)]
pub enum ExpressionToken<'a> {
    Block(Block<'a>),
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
    Block(Block<'a>),
    Function(&'a str, Block<'a>),
    Expression(Expression<'a>),
    Assignment(Variable, Box<[Self]>),
    Return(Box<[Self]>),
    PrintExpression(Expression<'a>),
    PrintAssignment(Variable, Box<[Self]>),
    PrintReturn(Box<[Self]>),
    Last(Box<[Self]>),
}
#[derive(Clone, Debug)]
pub enum ParseError {
    MissingInput,
    TooMuchInput,
    LexerError,
}
impl<'a> Block<'a> {
    pub fn parse(options: ParserOptions<'_>, s: &'a str) -> Result<Self, ParseError> {
        let mut tokens = peek_nth(LexerIter::new(s));
        Self::parse_tokens(options, &mut tokens)
    }
    pub fn parse_tokens(
        options: ParserOptions<'_>,
        tokens: &mut PeekNth<LexerIter<'a>>,
    ) -> Result<Self, ParseError> {
        let mut blocks = Vec::new();
        while let Some(token) = tokens.peek() {
            let part = match token.token {
                LexerToken::UnexpectedEnd => return Err(ParseError::LexerError),
                LexerToken::Word("fn") => todo!(),
                LexerToken::Word("let") => todo!(),
                LexerToken::Word("mut") => todo!(),
                LexerToken::Word("return") => todo!(),
                LexerToken::LeftParenthesis => todo!(),
                _ => BlockToken::parse_expression(options, tokens)?,
            };
            blocks.push(part);
        }
        Ok(Self {
            blocks: blocks.into_boxed_slice(),
        })
    }
}
pub(crate) struct State<'a> {
    pub expression: Vec<ExpressionToken<'a>>,
    pub operator_stack: Vec<OperatorToken>,
    pub fn_inputs: Vec<u32>,
    pub input_left: bool,
}
#[expect(clippy::derivable_impls)]
impl Default for State<'_> {
    fn default() -> Self {
        Self {
            expression: Vec::new(),
            operator_stack: Vec::new(),
            fn_inputs: Vec::new(),
            input_left: false,
        }
    }
}
impl State<'_> {
    fn push_operator(&mut self, operator: OperatorToken) {
        self.input_left = operator.is_unary() && !operator.unary_left();
        if matches!(operator, OperatorToken::LeftBracket(_)) {
            self.operator_stack.push(operator);
            return;
        }
        if matches!(operator, OperatorToken::Function(_)) {
            self.fn_inputs.push(0);
        }
        while let Some(top) = self.operator_stack.pop_if(|top| {
            !matches!(
                top,
                OperatorToken::LeftBracket(_) | OperatorToken::Function(_)
            ) && (top.precedence() > operator.precedence()
                || (top.precedence() == operator.precedence() && operator.left_associative()))
                && !(!self.input_left
                    && operator.is_unary()
                    && operator.unary_left()
                    && !top.left_associative()
                    && top.precedence() > operator.precedence())
        }) {
            self.expression
                .push(ExpressionToken::Function(Function::from(top)));
        }
        self.operator_stack.push(operator);
    }
    fn push_number(&mut self, number: NumberInner) {
        if self.input_left {
            self.push_operator(OperatorToken::Operator(Operator::MulInner));
        }
        self.input_left = true;
        self.expression
            .push(ExpressionToken::Number(Number::Complex(number)));
    }
    fn end(&mut self) -> Result<(), ParseError> {
        while let Some(operator) = self.operator_stack.pop() {
            if let OperatorToken::LeftBracket(bracket) = operator {
                self.close_off_bracket(bracket)?;
            } else {
                self.expression
                    .push(ExpressionToken::Function(Function::from(operator)));
            }
        }
        Ok(())
    }
    fn close_expr(&mut self) {
        while let Some(top) = self
            .operator_stack
            .pop_if(|top| !matches!(top, OperatorToken::LeftBracket(_)))
        {
            self.expression
                .push(ExpressionToken::Function(Function::from(top)));
        }
    }
    fn close_off_bracket(&mut self, bracket: Bracket) -> Result<(), ParseError> {
        self.close_expr();
        let Some(OperatorToken::LeftBracket(_)) = self
            .operator_stack
            .pop_if(|top| *top == OperatorToken::LeftBracket(bracket))
        else {
            return Ok(());
        };
        match bracket {
            Bracket::Absolute => {
                self.expression
                    .push(ExpressionToken::Function(Function::Abs));
            }
            Bracket::Parenthesis => {
                if let Some(OperatorToken::Function(fun)) = self
                    .operator_stack
                    .pop_if(|o| matches!(o, OperatorToken::Function(_)))
                {
                    let mut inputs = self.fn_inputs.pop().unwrap();
                    if self.input_left {
                        inputs += 1;
                    }
                    if !fun.input_range().contains(&inputs) {
                        return Err(if fun.input_range().start < inputs {
                            ParseError::MissingInput
                        } else {
                            ParseError::TooMuchInput
                        });
                    }
                    self.expression.push(ExpressionToken::Function(fun));
                }
            }
            Bracket::Square => {
                let mut inputs = self.fn_inputs.pop().unwrap();
                if self.input_left {
                    inputs += 1;
                }
                self.expression
                    .push(ExpressionToken::Function(Function::Vector(inputs)));
            }
        }
        self.input_left = true;
        Ok(())
    }
}
impl<'a> BlockToken<'a> {
    pub fn parse_expression(
        options: ParserOptions<'_>,
        tokens: &mut PeekNth<LexerIter<'a>>,
    ) -> Result<Self, ParseError> {
        let mut state = State::default();
        let mut print = true;
        while let Some(token) = tokens.next() {
            match token.token {
                LexerToken::UnexpectedEnd => return Err(ParseError::LexerError),
                LexerToken::BackSlash if tokens.next_if_eq(&LexerToken::NewLine).is_some() => {}
                LexerToken::LeftParenthesis => {
                    state.push_operator(OperatorToken::LeftBracket(Bracket::Parenthesis));
                }
                LexerToken::RightParenthesis => state.close_off_bracket(Bracket::Parenthesis)?,
                LexerToken::LeftSquareBracket => {
                    state.fn_inputs.push(0);
                    state.push_operator(OperatorToken::LeftBracket(Bracket::Square));
                }
                LexerToken::RightSquareBracket => state.close_off_bracket(Bracket::Square)?,
                LexerToken::Numeric(base, whole, part) => {
                    let number =
                        NumberInner::parse_number(base.unwrap_or(10), whole, part.unwrap_or(0));
                    state.push_number(number);
                }
                LexerToken::SemiColon => {
                    state.input_left = false;
                    print = false;
                }
                LexerToken::Word(str) if let Ok(fun) = Function::from_str(str) => {
                    state.push_operator(OperatorToken::Function(fun));
                }
                LexerToken::Comma => {
                    state.input_left = false;
                    state.close_expr();
                    if let Some(last) = state.fn_inputs.last_mut() {
                        *last = last.checked_add(1).unwrap();
                    }
                }
                LexerToken::Word(str) if let Some(num) = Dimension::parse(options, str) => {
                    state.push_number(num);
                }
                t if let Some(op) = OperatorToken::parse(&state, t, tokens) => {
                    state.push_operator(op);
                }
                _ => todo!(),
            }
        }
        state.end()?;
        let expression = Expression {
            tokens: state.expression.into_boxed_slice(),
        };
        Ok(if print {
            Self::PrintExpression(expression)
        } else {
            Self::Expression(expression)
        })
    }
}
