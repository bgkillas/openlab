use crate::functions::Function;
use crate::operators::{Bracket, Operator, OperatorToken};
use crate::options::ParserOptions;
use crate::units::{Dimension, UNIT_COUNT, UnitType};
use core::range::Range;
use core::str::FromStr as _;
use itertools::{PeekNth, peek_nth};
use lexer::iter::LexerIter;
use lexer::token::{LexerItem, LexerToken};
use number::float::complex::Complex;
use number::traits::number::NumberImpl as _;
use number::units::NumberUnits;
use std::debug_assert_matches;
pub type Float = f64;
pub type NumberBase = Complex<Float>;
pub type NumberInner = NumberUnits<NumberBase, UnitType, UNIT_COUNT>;
pub type Number = number::number::Number<NumberInner>;
#[derive(PartialEq, Clone, Default, Debug)]
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
#[derive(PartialEq, Clone, Debug)]
pub enum ExpressionToken<'a> {
    Block(Block<'a>),
    Number(Number),
    Variable(Variable),
    Function(Function),
}
#[derive(PartialEq, Clone, Default, Debug)]
pub struct Block<'a> {
    pub blocks: Box<[BlockToken<'a>]>,
}
#[derive(PartialEq, Clone, Debug)]
pub enum BlockToken<'a> {
    Block(Block<'a>),
    Function(&'a str, Block<'a>),
    Expression(Expression<'a>),
    Assignment(Variable, Expression<'a>),
    Return(Expression<'a>),
    PrintExpression(Expression<'a>),
    PrintAssignment(Variable, Expression<'a>),
    PrintReturn(Expression<'a>),
    If(Expression<'a>, Block<'a>, Option<Block<'a>>),
}
pub struct FormatStr<'a> {
    pub str: &'a str,
    pub elems: Box<[(Range<usize>, FormatOptions)]>,
}
pub struct FormatOptions;
#[derive(Clone, Debug)]
pub enum ParseError {
    MissingInput,
    TooMuchInput,
    LexerError,
    UnexpectedToken,
    NoBracketAfterFunction,
    BadFormat,
    NoVarNamed,
    IfExpectedExpression,
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
        let mut state = State::default();
        state.parse_block(options, tokens)
    }
}
pub(crate) struct State<'a> {
    pub expression: Vec<ExpressionToken<'a>>,
    pub operator_stack: Vec<OperatorToken>,
    pub fn_inputs: Vec<u32>,
    pub input_left: bool,
    pub variables: Vec<&'a str>,
}
#[expect(clippy::derivable_impls)]
impl Default for State<'_> {
    fn default() -> Self {
        Self {
            expression: Vec::new(),
            operator_stack: Vec::new(),
            fn_inputs: Vec::new(),
            input_left: false,
            variables: Vec::new(),
        }
    }
}
fn get_let_mut_vars<'a>(
    _: ParserOptions<'_>,
    tokens: &mut PeekNth<LexerIter<'a>>,
) -> Result<&'a str, ParseError> {
    let token = tokens
        .next()
        .map_or(Err(ParseError::MissingInput), |t| Ok(t.token))?;
    match token {
        LexerToken::Word(word) => Ok(word),
        _ => Err(ParseError::UnexpectedToken),
    }
}
impl<'a> State<'a> {
    fn parse_block(
        &mut self,
        options: ParserOptions<'_>,
        tokens: &mut PeekNth<LexerIter<'a>>,
    ) -> Result<Block<'a>, ParseError> {
        let mut blocks = Vec::new();
        while let Some(token) = tokens.peek() {
            let part = match token.token {
                LexerToken::UnexpectedEnd => return Err(ParseError::LexerError),
                LexerToken::Word("if") => self.parse_if(options, tokens)?,
                LexerToken::Word("else") => todo!(),
                LexerToken::SingleQuote => todo!(),
                LexerToken::Word("for") => todo!(),
                LexerToken::Word("in") => todo!(),
                LexerToken::Word("while") => todo!(),
                LexerToken::Word("loop") => todo!(),
                LexerToken::Word("break") => todo!(),
                LexerToken::Word("continue") => todo!(),
                LexerToken::Word("fn") => todo!(),
                LexerToken::Word("let") => self.parse_let(options, tokens)?,
                LexerToken::Word("mut") => self.parse_mut(options, tokens)?,
                LexerToken::Word("return") => todo!(),
                LexerToken::LeftCurlyBracket => {
                    BlockToken::Block(self.parse_block_inner(options, tokens)?)
                }
                LexerToken::RightCurlyBracket => {
                    tokens.next();
                    break;
                }
                _ => self.parse_expression(options, tokens)?,
            };
            blocks.push(part);
        }
        Ok(Block {
            blocks: blocks.into_boxed_slice(),
        })
    }
    fn parse_block_inner(
        &mut self,
        options: ParserOptions<'_>,
        tokens: &mut PeekNth<LexerIter<'a>>,
    ) -> Result<Block<'a>, ParseError> {
        let tkn = tokens.next();
        debug_assert_matches!(
            tkn,
            Some(LexerItem {
                token: LexerToken::LeftCurlyBracket,
                range: _
            })
        );
        let len = self.variables.len();
        let block = self.parse_block(options, tokens)?;
        self.variables.drain(len..);
        Ok(block)
    }
    fn parse_if(
        &mut self,
        options: ParserOptions<'_>,
        tokens: &mut PeekNth<LexerIter<'a>>,
    ) -> Result<BlockToken<'a>, ParseError> {
        let tkn = tokens.next();
        debug_assert_matches!(
            tkn,
            Some(LexerItem {
                token: LexerToken::Word("if"),
                range: _
            })
        );
        let BlockToken::Expression(expr) = self.parse_expression(options, tokens)? else {
            return Err(ParseError::IfExpectedExpression);
        };
        let block = self.parse_block_inner(options, tokens)?;
        let ifelse = if tokens
            .peek()
            .is_some_and(|t| t.token == LexerToken::Word("else"))
        {
            tokens.next();
            if tokens
                .peek()
                .is_some_and(|t| t.token == LexerToken::Word("if"))
            {
                let ifelse = self.parse_if(options, tokens)?;
                Some(Block {
                    blocks: Box::from([ifelse]),
                })
            } else {
                Some(self.parse_block_inner(options, tokens)?)
            }
        } else {
            None
        };
        Ok(BlockToken::If(expr, block, ifelse))
    }
    fn parse_let(
        &mut self,
        options: ParserOptions<'_>,
        tokens: &mut PeekNth<LexerIter<'a>>,
    ) -> Result<BlockToken<'a>, ParseError> {
        let tkn = tokens.next();
        debug_assert_matches!(
            tkn,
            Some(LexerItem {
                token: LexerToken::Word("let"),
                range: _
            })
        );
        let var_name = get_let_mut_vars(options, tokens)?;
        let var = Variable::Variable(self.variables.len().strict_cast());
        if tokens.next().is_none_or(|t| t.token != LexerToken::Equal) {
            return Err(ParseError::UnexpectedToken);
        }
        let expr = self.parse_expression(options, tokens)?;
        self.variables.push(var_name);
        Ok(match expr {
            BlockToken::Expression(exp) => BlockToken::Assignment(var, exp),
            BlockToken::PrintExpression(exp) => BlockToken::PrintAssignment(var, exp),
            _ => unreachable!(),
        })
    }
    fn parse_mut(
        &mut self,
        options: ParserOptions<'_>,
        tokens: &mut PeekNth<LexerIter<'a>>,
    ) -> Result<BlockToken<'a>, ParseError> {
        let tkn = tokens.next();
        debug_assert_matches!(
            tkn,
            Some(LexerItem {
                token: LexerToken::Word("mut"),
                range: _
            })
        );
        let var_name = get_let_mut_vars(options, tokens)?;
        let pos = self
            .variables
            .iter()
            .rposition(|s| *s == var_name)
            .ok_or(ParseError::NoVarNamed)?
            .strict_cast();
        let var = Variable::Variable(pos);
        if tokens.next().is_none_or(|t| t.token != LexerToken::Equal) {
            return Err(ParseError::UnexpectedToken);
        }
        let expr = self.parse_expression(options, tokens)?;
        Ok(match expr {
            BlockToken::Expression(exp) => BlockToken::Assignment(var, exp),
            BlockToken::PrintExpression(exp) => BlockToken::PrintAssignment(var, exp),
            _ => unreachable!(),
        })
    }
    fn push_operator(&mut self, operator: OperatorToken) {
        self.input_left = operator.is_unary() && !operator.unary_left();
        if matches!(operator, OperatorToken::LeftBracket(_)) {
            self.fn_inputs.push(0);
            self.operator_stack.push(operator);
            return;
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
    fn push_variable(&mut self, var: u32) {
        if self.input_left {
            self.push_operator(OperatorToken::Operator(Operator::MulInner));
        }
        self.input_left = true;
        self.expression
            .push(ExpressionToken::Variable(Variable::Variable(var)));
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
    fn optimize_matrix(&mut self, height: u32) -> Option<u32> {
        if height == 0 {
            return None;
        }
        let mut iter = self.expression.iter().rev();
        let width = {
            let &ExpressionToken::Function(Function::Vector(wid)) = iter.next()? else {
                return None;
            };
            wid
        };
        if height == 1 {
            self.expression.pop();
            return Some(width);
        }
        let mut inputs = height - 1;
        let mut inner_inputs = width;
        let mut pos = Vec::with_capacity(height.strict_cast());
        for (i, token) in iter.enumerate() {
            match token {
                ExpressionToken::Block(_)
                | ExpressionToken::Number(_)
                | ExpressionToken::Variable(_) => {
                    inner_inputs -= 1;
                }
                ExpressionToken::Function(f) => {
                    if inner_inputs == 0 {
                        let &Function::Vector(wid) = f else {
                            return None;
                        };
                        if wid != width {
                            return None;
                        }
                        pos.push(i);
                        if inputs == 0 {
                            break;
                        }
                        inputs -= 1;
                        inner_inputs = width;
                    } else {
                        inner_inputs += f.inputs() - 1;
                    }
                }
            }
        }
        self.expression.pop();
        let len = self.expression.len() - 1;
        for p in pos {
            self.expression.remove(len - p);
        }
        Some(width)
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
                let mut inputs = self.fn_inputs.pop().unwrap();
                if self.input_left {
                    inputs += 1;
                }
                if let Some(OperatorToken::Function(fun)) = self
                    .operator_stack
                    .pop_if(|o| matches!(o, OperatorToken::Function(_)))
                {
                    if !fun.input_range().contains(&inputs) {
                        return Err(if fun.input_range().start < inputs {
                            ParseError::MissingInput
                        } else {
                            ParseError::TooMuchInput
                        });
                    }
                    self.expression.push(ExpressionToken::Function(fun));
                } else if inputs != 1 || !self.input_left {
                    self.expression
                        .push(ExpressionToken::Function(Function::List(inputs)));
                }
            }
            Bracket::Square => {
                let mut inputs = self.fn_inputs.pop().unwrap();
                if self.input_left {
                    inputs += 1;
                }
                let vec = if let Some(width) = self.optimize_matrix(inputs) {
                    Function::Matrix(width, inputs)
                } else {
                    Function::Vector(inputs)
                };
                self.expression.push(ExpressionToken::Function(vec));
            }
        }
        self.input_left = true;
        Ok(())
    }
    pub fn parse_expression(
        &mut self,
        options: ParserOptions<'_>,
        tokens: &mut PeekNth<LexerIter<'a>>,
    ) -> Result<BlockToken<'a>, ParseError> {
        self.reset();
        let mut print = true;
        while let Some(token) = tokens.next() {
            match token.token {
                LexerToken::UnexpectedEnd => return Err(ParseError::LexerError),
                LexerToken::BackSlash if tokens.next_if_eq(&LexerToken::NewLine).is_some() => {}
                LexerToken::LeftParenthesis => {
                    self.push_operator(OperatorToken::LeftBracket(Bracket::Parenthesis));
                }
                LexerToken::RightParenthesis => self.close_off_bracket(Bracket::Parenthesis)?,
                LexerToken::LeftSquareBracket => {
                    self.push_operator(OperatorToken::LeftBracket(Bracket::Square));
                }
                LexerToken::RightSquareBracket => self.close_off_bracket(Bracket::Square)?,
                LexerToken::Numeric(base, whole, part) => {
                    let number =
                        NumberInner::parse_number(base.unwrap_or(10), whole, part.unwrap_or(0));
                    self.push_number(number);
                }
                LexerToken::NewLine => {
                    break;
                }
                LexerToken::SemiColon => {
                    print = false;
                    break;
                }
                LexerToken::Comma => {
                    self.input_left = false;
                    self.close_expr();
                    if let Some(last) = self.fn_inputs.last_mut() {
                        *last = last.checked_add(1).unwrap();
                    }
                }
                LexerToken::Word(str) if let Ok(fun) = Function::from_str(str) => {
                    self.push_operator(OperatorToken::Function(fun));
                }
                LexerToken::Word(str)
                    if let Some(var) = self.variables.iter().rposition(|&s| s == str) =>
                {
                    self.push_variable(var.strict_cast());
                }
                LexerToken::Word(str) if let Some(num) = Dimension::parse(options, str) => {
                    self.push_number(num);
                }
                t if let Some(op) = OperatorToken::parse(self, t, tokens) => {
                    self.push_operator(op);
                }
                _ => return Err(ParseError::UnexpectedToken),
            }
        }
        self.end()?;
        let expression = Expression {
            tokens: self.expression.drain(..).collect(),
        };
        self.reset();
        Ok(if print {
            BlockToken::PrintExpression(expression)
        } else {
            BlockToken::Expression(expression)
        })
    }
    pub fn reset(&mut self) {
        self.input_left = false;
        debug_assert_eq!(self.expression, []);
        debug_assert_eq!(self.operator_stack, []);
        debug_assert_eq!(self.fn_inputs, [0; 0]);
    }
}
impl<'a> BlockToken<'a> {
    pub fn parse_expression_str(
        options: ParserOptions<'_>,
        s: &'a str,
    ) -> Result<Self, ParseError> {
        let mut tokens = peek_nth(LexerIter::new(s));
        let mut state = State::default();
        state.parse_expression(options, &mut tokens)
    }
}
impl<'a> FormatStr<'a> {
    pub fn get_string<I>(&self, iter: &mut I) -> String
    where
        I: Iterator<Item = Number>,
    {
        let mut string = String::new();
        let mut start = 0;
        for (n, &(range, _)) in iter.zip(&self.elems) {
            string.push_str(&self.str[start..range.start]);
            string.push_str(format!("{n:?}").as_str());
            start = range.end;
        }
        string.push_str(&self.str[start..]);
        string
    }
    pub fn new(str: &'a str) -> Result<Self, ParseError> {
        let mut elems = Vec::new();
        let mut is_left = None;
        for (i, c) in str.char_indices() {
            match c {
                '{' => is_left = Some(i),
                '}' => {
                    if let Some(left) = is_left {
                        elems.push((
                            Range::from(left..(i + 1)),
                            FormatOptions::new(&str[left + 1..i]),
                        ));
                    } else {
                        return Err(ParseError::BadFormat);
                    }
                    is_left = None;
                }
                _ => {}
            }
        }
        Ok(Self {
            str,
            elems: elems.into_boxed_slice(),
        })
    }
}
impl FormatOptions {
    pub fn new(_: &str) -> Self {
        Self
    }
}
