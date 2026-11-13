use crate::functions::Function;
use crate::operators::{Bracket, Operator, OperatorToken};
use crate::options::ParserOptions;
use crate::types::Type;
use crate::units::{Dimension, UNIT_COUNT, UnitType};
use core::range::Range;
use core::str::FromStr as _;
use itertools::{PeekNth, peek_nth};
use lexer::iter::LexerIter;
use lexer::token::LexerToken;
use number::float::complex::Complex;
use number::traits::number::NumberImpl as _;
use number::units::NumberUnits;
use std::debug_assert_matches;
pub type Float = f64;
pub type NumberBase = Complex<Float>;
pub type Number = NumberUnits<NumberBase, UnitType, UNIT_COUNT>;
#[derive(PartialEq, Clone, Default, Debug)]
pub struct Expression {
    pub tokens: Box<[ExpressionToken]>,
}
#[derive(PartialEq, Clone, Debug)]
pub enum ExpressionToken {
    Block(Block),
    Type(Type),
    Variable(u32),
    Function(Function),
}
#[derive(PartialEq, Clone, Default, Debug)]
pub struct Block {
    pub blocks: Box<[BlockToken]>,
}
#[derive(PartialEq, Clone, Debug)]
pub enum BlockToken {
    Block(Block),
    Function(u32, u32, Block),
    Expression(Expression),
    LetAssignment(Expression),
    Assignment(u32, Expression),
    Return(Expression),
    PrintExpression(Expression),
    PrintAssignment(u32, Expression),
    PrintLetAssignment(Expression),
    PrintReturn(Expression),
    If(Expression, Block, Option<Block>),
}
pub struct FormatStr<'a> {
    pub str: &'a str,
    pub elems: Box<[(Range<usize>, FormatOptions)]>,
}
pub struct FormatOptions;
#[derive(Clone, Debug)]
pub enum ParseErrorInner {
    MissingInput,
    TooMuchInput,
    LexerError,
    UnexpectedToken,
    NoBracketAfterFunction,
    BadFormat,
    NoVarNamed,
    IfExpectedExpression,
    ExpectedFnName,
}
impl ParseErrorInner {
    pub fn range<R>(self, range: R) -> ParseError
    where
        R: Into<Range<usize>>,
    {
        ParseError {
            inner: self,
            range: range.into(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct ParseError {
    pub inner: ParseErrorInner,
    pub range: Range<usize>,
}
impl Block {
    pub fn parse(options: ParserOptions<'_>, s: &str) -> Result<Self, ParseError> {
        let mut tokens = peek_nth(LexerIter::new(s));
        Self::parse_tokens(options, &mut tokens)
    }
    pub fn parse_tokens(
        options: ParserOptions<'_>,
        tokens: &mut PeekNth<LexerIter<'_>>,
    ) -> Result<Self, ParseError> {
        let mut state = State::default();
        state.parse_block(options, tokens)
    }
}
#[derive(Debug)]
pub struct State<'a> {
    pub expression: Vec<ExpressionToken>,
    pub operator_stack: Vec<OperatorToken>,
    pub fn_inputs: Vec<u32>,
    pub input_left: bool,
    pub variables: Vec<&'a str>,
    pub funs: Vec<(&'a str, u32)>,
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
            funs: Vec::new(),
        }
    }
}
fn custom_input_range(state: &State) -> impl FnOnce(u32) -> core::ops::Range<u32> {
    |c: u32| {
        let (_, n) = state.funs[c.strict_cast::<usize>()];
        n..n + 1
    }
}
fn custom_inputs(state: &State) -> impl FnOnce(u32) -> u32 {
    |c: u32| {
        let (_, n) = state.funs[c.strict_cast::<usize>()];
        n
    }
}
fn get_let_mut_vars<'a>(
    _: ParserOptions<'_>,
    tokens: &mut PeekNth<LexerIter<'a>>,
) -> Result<&'a str, ParseError> {
    let Some(token) = tokens.next() else {
        return Err(ParseErrorInner::MissingInput.range(0..0));
    };
    if let LexerToken::Word(word) = token.1 {
        Ok(word)
    } else {
        Err(ParseErrorInner::UnexpectedToken.range(token.0))
    }
}
impl<'a> State<'a> {
    fn parse_block(
        &mut self,
        options: ParserOptions<'_>,
        tokens: &mut PeekNth<LexerIter<'a>>,
    ) -> Result<Block, ParseError> {
        let mut blocks = Vec::new();
        while let Some(token) = tokens.peek() {
            let part = match token.1 {
                LexerToken::UnexpectedEnd => {
                    return Err(ParseErrorInner::LexerError.range(token.0));
                }
                LexerToken::Word("if") => self.parse_if(options, tokens)?,
                LexerToken::Word("else") => todo!(),
                LexerToken::SingleQuote => todo!(),
                LexerToken::Word("for") => todo!(),
                LexerToken::Word("in") => todo!(),
                LexerToken::Word("while") => todo!(),
                LexerToken::Word("loop") => todo!(),
                LexerToken::Word("break") => todo!(),
                LexerToken::Word("continue") => todo!(),
                LexerToken::Word("fn") => self.parse_fn(options, tokens)?,
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
                LexerToken::NewLine => continue,
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
    ) -> Result<Block, ParseError> {
        let tkn = tokens.next();
        debug_assert_matches!(tkn, Some((_, LexerToken::LeftCurlyBracket)));
        let len = self.variables.len();
        let fun_len = self.funs.len();
        let block = self.parse_block(options, tokens)?;
        self.variables.drain(len..);
        self.funs.drain(fun_len..);
        Ok(block)
    }
    fn parse_fn(
        &mut self,
        options: ParserOptions<'_>,
        tokens: &mut PeekNth<LexerIter<'a>>,
    ) -> Result<BlockToken, ParseError> {
        let tkn = tokens.next();
        debug_assert_matches!(tkn, Some((_, LexerToken::Word("fn"))));
        let Some((_, LexerToken::Word(fn_name))) = tokens.next() else {
            return Err(ParseErrorInner::ExpectedFnName.range(0..0));
        };
        let Some((_, LexerToken::LeftParenthesis)) = tokens.next() else {
            return Err(ParseErrorInner::UnexpectedToken.range(0..0));
        };
        let mut vars: u32 = 0;
        while let Some((_, LexerToken::Word(var))) =
            tokens.next_if(|(_, t)| matches!(t, LexerToken::Word(_)))
        {
            vars += 1;
            self.variables.push(var);
            match tokens.next() {
                Some((_, LexerToken::Comma)) => {}
                Some((_, LexerToken::RightParenthesis)) => break,
                Some((r, _)) => return Err(ParseErrorInner::UnexpectedToken.range(r)),
                None => return Err(ParseErrorInner::MissingInput.range(0..0)),
            }
        }
        let fn_var = self.funs.len().strict_cast();
        self.funs.push((fn_name, vars));
        let block = self.parse_block_inner(options, tokens)?;
        self.variables
            .drain(self.variables.len() - vars.strict_cast::<usize>()..);
        Ok(BlockToken::Function(fn_var, vars, block))
    }
    fn parse_if(
        &mut self,
        options: ParserOptions<'_>,
        tokens: &mut PeekNth<LexerIter<'a>>,
    ) -> Result<BlockToken, ParseError> {
        let tkn = tokens.next();
        debug_assert_matches!(tkn, Some((_, LexerToken::Word("if"))));
        let BlockToken::PrintExpression(expr) = self.parse_expression(options, tokens)? else {
            return Err(ParseErrorInner::IfExpectedExpression.range(0..0));
        };
        let block = self.parse_block_inner(options, tokens)?;
        let ifelse = if tokens
            .peek()
            .is_some_and(|&(_, t)| t == LexerToken::Word("else"))
        {
            tokens.next();
            if tokens
                .peek()
                .is_some_and(|&(_, t)| t == LexerToken::Word("if"))
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
    ) -> Result<BlockToken, ParseError> {
        let tkn = tokens.next();
        debug_assert_matches!(tkn, Some((_, LexerToken::Word("let"))));
        let var_name = get_let_mut_vars(options, tokens)?;
        if tokens.next().is_none_or(|(_, t)| t != LexerToken::Equal) {
            return Err(ParseErrorInner::UnexpectedToken.range(0..0));
        }
        let expr = self.parse_expression(options, tokens)?;
        self.variables.push(var_name);
        Ok(match expr {
            BlockToken::Expression(exp) => BlockToken::LetAssignment(exp),
            BlockToken::PrintExpression(exp) => BlockToken::PrintLetAssignment(exp),
            _ => unreachable!(),
        })
    }
    fn parse_mut(
        &mut self,
        options: ParserOptions<'_>,
        tokens: &mut PeekNth<LexerIter<'a>>,
    ) -> Result<BlockToken, ParseError> {
        let tkn = tokens.next();
        debug_assert_matches!(tkn, Some((_, LexerToken::Word("mut"))));
        let var_name = get_let_mut_vars(options, tokens)?;
        let pos = self
            .variables
            .iter()
            .rposition(|s| *s == var_name)
            .ok_or(ParseErrorInner::NoVarNamed.range(0..0))?
            .strict_cast();
        if tokens.next().is_none_or(|(_, t)| t != LexerToken::Equal) {
            return Err(ParseErrorInner::UnexpectedToken.range(0..0));
        }
        let expr = self.parse_expression(options, tokens)?;
        Ok(match expr {
            BlockToken::Expression(exp) => BlockToken::Assignment(pos, exp),
            BlockToken::PrintExpression(exp) => BlockToken::PrintAssignment(pos, exp),
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
        if matches!(operator, OperatorToken::Function(_)) {
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
        self.expression.push(ExpressionToken::Variable(var));
    }
    fn push_number(&mut self, number: Number) {
        if self.input_left {
            self.push_operator(OperatorToken::Operator(Operator::MulInner));
        }
        self.input_left = true;
        self.expression
            .push(ExpressionToken::Type(Type::Number(number)));
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
                | ExpressionToken::Type(_)
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
                        inner_inputs += f.inputs(custom_inputs(self)) - 1;
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
                    if !fun.input_range(custom_input_range(self)).contains(&inputs) {
                        return Err(
                            if fun.input_range(custom_input_range(self)).start < inputs {
                                ParseErrorInner::MissingInput.range(0..0)
                            } else {
                                ParseErrorInner::TooMuchInput.range(0..0)
                            },
                        );
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
    ) -> Result<BlockToken, ParseError> {
        self.reset();
        let mut print = true;
        loop {
            if self.input_left
                && tokens.peek().is_some_and(|(_, tkn)| {
                    matches!(
                        tkn,
                        LexerToken::LeftCurlyBracket | LexerToken::RightCurlyBracket
                    )
                })
            {
                break;
            }
            let Some((range, token)) = tokens.next() else {
                break;
            };
            match token {
                LexerToken::UnexpectedEnd => return Err(ParseErrorInner::LexerError.range(range)),
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
                    let number = Number::parse_number(base.unwrap_or(10), whole, part.unwrap_or(0));
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
                LexerToken::Word(str)
                    if let Some(var) = self.funs.iter().rposition(|&(s, _)| s == str) =>
                {
                    self.push_operator(OperatorToken::Function(Function::Custom(
                        var.strict_cast(),
                    )));
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
                _ => return Err(ParseErrorInner::UnexpectedToken.range(range)),
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
impl BlockToken {
    pub fn parse_expression_str(options: ParserOptions<'_>, s: &str) -> Result<Self, ParseError> {
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
                        return Err(ParseErrorInner::BadFormat.range(0..0));
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
