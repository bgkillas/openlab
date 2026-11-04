use core::iter;
use core::mem;
use core::ops::{Add, Div, Mul, Sub};
use core::ops::{ControlFlow, FromResidual, Residual, Try};
use parser::functions::Function;
use parser::number::matrix::dense::Matrix;
use parser::number::matrix::size::MatrixDimension;
use parser::number::traits::pow::{Pow, Root};
use parser::operators::Operator;
use parser::options::ParserOptions;
use parser::parser::{
    Block, BlockToken, Expression, ExpressionToken, Number, ParseError, Variable,
};
use std::io::{Empty, Stdout, Write, empty, stdout};
#[derive(Debug, Default)]
struct ComputeInner<S: Write> {
    stack: Vec<Number>,
    stdout: S,
}
#[derive(Debug)]
pub struct Compute<'a, S: Write> {
    block: Block<'a>,
    inner: ComputeInner<S>,
}
impl<'a, S: Write> Compute<'a, S> {
    pub fn parse(options: ParserOptions<'_>, s: &'a str, stdout: S) -> Result<Self, ParseError> {
        Ok(Self::from_block(Block::parse(options, s)?, stdout))
    }
    pub fn from_block(block: Block<'a>, stdout: S) -> Self {
        Self {
            block,
            inner: ComputeInner::new(stdout),
        }
    }
    pub fn compute(&mut self) -> Option<Number> {
        let old = self.inner.stack.len();
        let ret = self.inner.block(&self.block);
        debug_assert_eq!(self.inner.stack.len(), old);
        ret.option()
    }
}
impl<'a> Compute<'a, Vec<u8>> {
    pub fn parse_str(options: ParserOptions<'_>, s: &'a str) -> Result<Self, ParseError> {
        Ok(Self::from_block_str(Block::parse(options, s)?))
    }
    pub fn from_block_str(block: Block<'a>) -> Self {
        Self {
            block,
            inner: ComputeInner::new(Vec::new()),
        }
    }
    pub fn compute_str(&mut self) -> (Option<Number>, String) {
        let ret = self.compute();
        (
            ret,
            String::from_utf8(mem::take(&mut self.inner.stdout)).unwrap(),
        )
    }
}
impl<'a> Compute<'a, Stdout> {
    pub fn parse_stdout(options: ParserOptions<'_>, s: &'a str) -> Result<Self, ParseError> {
        Ok(Self::from_block_stdout(Block::parse(options, s)?))
    }
    pub fn from_block_stdout(block: Block<'a>) -> Self {
        Self {
            block,
            inner: ComputeInner::new(stdout()),
        }
    }
}
impl<'a> Compute<'a, Empty> {
    pub fn parse_empty(options: ParserOptions<'_>, s: &'a str) -> Result<Self, ParseError> {
        Ok(Self::from_block_empty(Block::parse(options, s)?))
    }
    pub fn from_block_empty(block: Block<'a>) -> Self {
        Self {
            block,
            inner: ComputeInner::new(empty()),
        }
    }
}
macro_rules! write_stdout {
    ($ident:ident, $($tt:tt)*) => {
        $ident.stdout.write_fmt(format_args!($($tt)*)).unwrap();
    };
}
impl<S: Write> ComputeInner<S> {
    pub fn new(stdout: S) -> Self {
        Self {
            stack: Vec::new(),
            stdout,
        }
    }
    pub fn block(&mut self, block: &Block<'_>) -> ReturnType<Number> {
        let len = self.stack.len();
        let ret = self.block_inner(block);
        self.stack.drain(len..);
        ret
    }
    fn block_inner(&mut self, block: &Block<'_>) -> ReturnType<Number> {
        for (i, block_token) in block.blocks.iter().enumerate() {
            match block_token {
                BlockToken::Expression(expr) => {
                    self.expr(expr)?;
                }
                BlockToken::PrintExpression(expr) => {
                    let res = self.expr(expr)?;
                    if i == block.blocks.len() - 1 {
                        return ReturnType::Last(res);
                    }
                    write_stdout!(self, "{res:?}");
                }
                BlockToken::Block(b) => {
                    let res = self.block(b);
                    if i == block.blocks.len() - 1 {
                        return ReturnType::Last(res?);
                    } else if !matches!(res, ReturnType::None) {
                        res?;
                    }
                }
                BlockToken::Function(_, _) => todo!(),
                BlockToken::Assignment(var, expr) => {
                    let number = self.expr(expr)?;
                    self.var(*var, number);
                }
                BlockToken::PrintAssignment(var, expr) => {
                    let number = self.expr(expr)?;
                    write_stdout!(self, "{number:?}");
                    self.var(*var, number);
                }
                BlockToken::Return(expr) => {
                    return ReturnType::Return(if expr.tokens.is_empty() {
                        None
                    } else {
                        self.expr(expr).option()
                    });
                }
                BlockToken::PrintReturn(expr) => {
                    return ReturnType::Return(if expr.tokens.is_empty() {
                        None
                    } else {
                        let ret = self.expr(expr).option();
                        if let Some(res) = &ret {
                            write_stdout!(self, "{res:?}");
                        }
                        ret
                    });
                }
            }
        }
        ReturnType::None
    }
    fn var(&mut self, var: Variable, number: Number) {
        match var {
            Variable::Variable(v) => {
                if self.stack.len() == v.strict_cast::<usize>() {
                    self.stack.push(number);
                } else {
                    self.stack[v.strict_cast::<usize>()] = number;
                }
            }
            Variable::Function(_) => todo!(),
            Variable::Global(_) => todo!(),
            Variable::None => todo!(),
        }
    }
    fn expr(&mut self, expr: &Expression) -> ReturnType<Number> {
        let len = self.stack.len();
        for token in &expr.tokens {
            let number = match token {
                ExpressionToken::Number(num) => num.clone(),
                &ExpressionToken::Function(fun) => {
                    self.fun(fun).map_or_default(ReturnType::Last)?
                }
                &ExpressionToken::Variable(Variable::Variable(v)) => {
                    self.stack[v.strict_cast::<usize>()].clone()
                }
                ExpressionToken::Variable(_) => todo!(),
                ExpressionToken::Block(block) => self.block(block)?,
            };
            self.stack.push(number);
        }
        let res = self.stack.pop();
        debug_assert_eq!(self.stack.len(), len);
        res.map_or_default(ReturnType::Last)
    }
    fn fun(&mut self, fun: Function) -> Option<Number> {
        let mut inputs = self.stack.drain(
            self.stack
                .len()
                .checked_sub(fun.inputs().strict_cast::<usize>())?..,
        );
        let mut next = || inputs.next().unwrap();
        match fun {
            Function::Vector(i) => {
                if i == 0 {
                    Some(Number::Matrix(Matrix::default()))
                } else {
                    let first = next();
                    let mat = match first {
                        Number::Complex(c) => {
                            let dim = MatrixDimension::new(i, 1);
                            let mut new_iter = iter::once(Number::Complex(c)).chain(inputs);
                            Matrix::new_with_try(dim, |_, _| new_iter.next()?.complex())?
                        }
                        Number::Matrix(m) if m.dimension.height == 1 => {
                            let dim = MatrixDimension::new(m.dimension.width, i);
                            let mut new_iter = iter::once(Number::Matrix(m)).chain(inputs);
                            Matrix::from_rows_try(dim, |_, _| new_iter.next()?.matrix_owned())?
                        }
                        Number::Matrix(m) if m.dimension.width == 1 => {
                            let dim = MatrixDimension::new(i, m.dimension.height);
                            let mut new_iter = iter::once(Number::Matrix(m)).chain(inputs);
                            Matrix::from_cols_try(dim, |_, _| new_iter.next()?.matrix_owned())?
                        }
                        Number::Matrix(_) | Number::List(_) => return None,
                    };
                    Some(Number::Matrix(mat))
                }
            }
            Function::List(i) => {
                if i == 0 {
                    Some(Number::List(Vec::new()))
                } else {
                    let vec = iter::repeat_with(next).take(i.strict_cast()).collect();
                    Some(Number::List(vec))
                }
            }
            Function::Matrix(i, j) => {
                if i == 0 || j == 0 {
                    Some(Number::Matrix(Matrix::default()))
                } else {
                    let dim = MatrixDimension::new(i, j);
                    let mat = Matrix::new_with_try(dim, |_, _| inputs.next()?.complex())?;
                    Some(Number::Matrix(mat))
                }
            }
            Function::AssertEq => {
                let a = next();
                let b = next();
                if a != b {
                    write_stdout!(self, "{a:?} does not equal {b:?}");
                }
                None
            }
            Function::Operator(o) => match o {
                Operator::Add => next() + next(),
                Operator::Sub => next() - next(),
                Operator::Mul | Operator::MulInner => next() * next(),
                Operator::Div => next() / next(),
                Operator::Pow => next().pow(&next()),
                Operator::Root => next().root(&next()),
                Operator::Identity => Some(next()),
                Operator::Negate => Some(-next()),
                Operator::Transpose => next().transpose(),
                Operator::Factorial => todo!(),
                Operator::SubFactorial => todo!(),
                Operator::Mod => todo!(),
            },
            Function::ComponentWise(o) => match o {
                Operator::Add => next().component_wise(&next(), Add::add),
                Operator::Sub => next().component_wise(&next(), Sub::sub),
                Operator::Mul | Operator::MulInner => next().component_wise(&next(), Mul::mul),
                Operator::Div => next().component_wise(&next(), Div::div),
                Operator::Pow => next().component_wise(&next(), Pow::pow),
                Operator::Root => next().component_wise(&next(), Root::root),
                Operator::Factorial => todo!(),
                Operator::SubFactorial => todo!(),
                Operator::Mod => todo!(),
                Operator::Identity | Operator::Negate | Operator::Transpose => unreachable!(),
            },
            _ => todo!(),
        }
    }
}
#[derive(Clone, Copy, Default, Debug)]
pub enum ReturnType<T> {
    Last(T),
    Return(Option<T>),
    Break(u32, Option<T>),
    Continue(u32),
    #[default]
    None,
}
impl<T> ReturnType<T> {
    pub fn option(self) -> Option<T> {
        match self {
            Self::Last(r) | Self::Return(Some(r)) | Self::Break(_, Some(r)) => Some(r),
            _ => None,
        }
    }
    pub fn is_return(&self) -> bool {
        matches!(self, Self::Return(_))
    }
    pub fn is_break(&self) -> bool {
        matches!(self, Self::Break(_, _))
    }
    pub fn is_continue(&self) -> bool {
        matches!(self, Self::Continue(_))
    }
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}
impl<T> Residual<T> for ReturnType<T> {
    type TryType = Self;
}
impl<T> FromResidual<Self> for ReturnType<T> {
    fn from_residual(residual: Self) -> Self {
        residual
    }
}
impl<T> Try for ReturnType<T> {
    type Output = T;
    type Residual = Self;
    fn from_output(output: Self::Output) -> Self {
        Self::Last(output)
    }
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Last(c) => ControlFlow::Continue(c),
            c @ (Self::Return(_) | Self::Break(_, _) | Self::Continue(_)) => ControlFlow::Break(c),
            Self::None => ControlFlow::Break(Self::None),
        }
    }
}
