use core::iter;
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
#[derive(Debug, Default)]
struct ComputeInner {
    stack: Vec<Number>,
}
#[derive(Debug)]
pub struct Compute<'a> {
    block: Block<'a>,
    inner: ComputeInner,
}
impl<'a> Compute<'a> {
    pub fn parse(options: ParserOptions<'_>, s: &'a str) -> Result<Self, ParseError> {
        Ok(Self {
            block: Block::parse(options, s)?,
            inner: ComputeInner::default(),
        })
    }
    pub fn from_block(block: Block<'a>) -> Self {
        Self {
            block,
            inner: ComputeInner::default(),
        }
    }
    pub fn compute(&mut self) -> Option<Number> {
        let old = self.inner.stack.len();
        let ret = self.inner.block(&self.block);
        debug_assert_eq!(self.inner.stack.len(), old);
        ret.option()
    }
}
impl ComputeInner {
    pub fn block(&mut self, block: &Block<'_>) -> ReturnType<Number> {
        let len = self.stack.len();
        let mut var_count = 0;
        let ret = self.block_inner(block, &mut var_count);
        self.stack.drain(self.stack.len() - var_count..);
        debug_assert_eq!(self.stack.len(), len);
        ret
    }
    fn block_inner(&mut self, block: &Block<'_>, var_count: &mut usize) -> ReturnType<Number> {
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
                    println!("{res:?}");
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
                    self.var(*var, number, var_count);
                }
                BlockToken::PrintAssignment(var, expr) => {
                    let number = self.expr(expr)?;
                    println!("{number:?}");
                    self.var(*var, number, var_count);
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
                            println!("{res:?}");
                        }
                        ret
                    });
                }
            }
        }
        ReturnType::None
    }
    fn var(&mut self, var: Variable, number: Number, var_count: &mut usize) {
        match var {
            Variable::Variable(v) => {
                if self.stack.len() == v.strict_cast::<usize>() {
                    *var_count += 1;
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
                    println!("{a:?} does not equal {b:?}");
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
