use core::iter;
use parser::functions::Function;
use parser::number::matrix::dense::Matrix;
use parser::number::matrix::size::MatrixDimension;
use parser::number::traits::pow::{Pow as _, Root as _};
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
    pub fn compute(&mut self) {
        for block in &self.block.blocks {
            match block {
                BlockToken::Expression(expr) => {
                    self.inner.expr(expr);
                }
                BlockToken::PrintExpression(expr) => {
                    let res = self.inner.expr(expr);
                    println!("{res:?}");
                }
                _ => todo!(),
            }
        }
    }
}
impl ComputeInner {
    fn expr(&mut self, expr: &Expression) -> Option<Number> {
        for token in &expr.tokens {
            let number = match token {
                ExpressionToken::Number(num) => num.clone(),
                &ExpressionToken::Function(fun) => self.fun(fun)?,
                ExpressionToken::Variable(_) => todo!(),
                ExpressionToken::Skip(_) => todo!(),
                ExpressionToken::Block(_) => todo!(),
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
            Function::Add => next() + next(),
            Function::Sub => next() - next(),
            Function::Mul => next() * next(),
            Function::Div => next() / next(),
            Function::Pow => next().pow(&next()),
            Function::Root => next().root(&next()),
            Function::Identity => Some(next()),
            Function::Negate => Some(-next()),
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
            Function::Transpose => next().transpose(),
            _ => todo!(),
        }
    }
}
