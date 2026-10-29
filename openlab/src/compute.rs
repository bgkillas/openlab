use core::iter;
use core::ops::{Add, Div, Mul, Sub};
use parser::functions::Function;
use parser::number::matrix::dense::Matrix;
use parser::number::matrix::size::MatrixDimension;
use parser::number::traits::pow::{Pow, Root};
use parser::operators::Operator;
use parser::options::ParserOptions;
use parser::parser::{Block, BlockToken, Expression, ExpressionToken, Number, ParseError};
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
        ret
    }
}
impl ComputeInner {
    pub fn block(&mut self, block: &Block<'_>) -> Option<Number> {
        for (i, block_token) in block.blocks.iter().enumerate() {
            match block_token {
                BlockToken::Expression(expr) => {
                    self.expr(expr);
                }
                BlockToken::PrintExpression(expr) => {
                    let res = self.expr(expr);
                    if i == block.blocks.len() - 1 {
                        return res;
                    }
                    println!("{res:?}");
                }
                _ => todo!(),
            }
        }
        None
    }
    fn expr(&mut self, expr: &Expression) -> Option<Number> {
        for token in &expr.tokens {
            let number = match token {
                ExpressionToken::Number(num) => num.clone(),
                &ExpressionToken::Function(fun) => self.fun(fun)?,
                ExpressionToken::Variable(_) => todo!(),
                ExpressionToken::Skip(_) => todo!(),
                ExpressionToken::Block(block) => self.block(block)?,
            };
            self.stack.push(number);
        }
        let res = self.stack.pop();
        debug_assert_eq!(self.stack, []);
        res
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
            Function::Matrix(i, j) => {
                if i == 0 || j == 0 {
                    Some(Number::Matrix(Matrix::default()))
                } else {
                    let dim = MatrixDimension::new(i, j);
                    let mat = Matrix::new_with_try(dim, |_, _| inputs.next()?.complex())?;
                    Some(Number::Matrix(mat))
                }
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
