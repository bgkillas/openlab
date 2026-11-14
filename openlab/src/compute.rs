use core::iter;
use core::mem;
use core::ops::{Add, Div, Mul, Sub};
use core::ops::{ControlFlow, FromResidual, Residual, Try};
use parser::functions::Function;
use parser::number::matrix::dense::Matrix;
use parser::number::matrix::size::MatrixDimension;
use parser::number::number::Number;
use parser::number::traits::number::NumberImpl as _;
use parser::number::traits::pow::{Pow, Root};
use parser::operators::Operator;
use parser::parser::{Block, BlockToken, Expression, ExpressionToken};
use parser::types::Type;
use std::io::{Stdout, Write, stdout};
#[derive(Debug, Default)]
pub struct Compute<'a, S: Write> {
    stack_start: usize,
    stack: Vec<Type>,
    funs: Vec<(u32, &'a Block)>,
    stdout: S,
}
impl Compute<'_, Stdout> {
    pub fn stdout() -> Self {
        Self {
            stack_start: 0,
            stack: Vec::new(),
            funs: Vec::new(),
            stdout: stdout(),
        }
    }
}
impl<'a> Compute<'a, Vec<u8>> {
    pub fn compute_str(&mut self, block: &'a Block) -> (Option<Type>, String) {
        let ret = self.compute(block);
        (ret, String::from_utf8(mem::take(&mut self.stdout)).unwrap())
    }
}
macro_rules! write_stdout {
    ($ident:ident, $($tt:tt)*) => {
        $ident.stdout.write_fmt(format_args!($($tt)*)).unwrap();
    };
}
impl<'a, S: Write> Compute<'a, S> {
    pub fn compute(&mut self, block: &'a Block) -> Option<Type> {
        let old = self.stack.len();
        let ret = self.block(block);
        debug_assert_eq!(self.stack.len(), old);
        ret.option()
    }
    pub fn new(stdout: S) -> Self {
        Self {
            stack_start: 0,
            stack: Vec::new(),
            funs: Vec::new(),
            stdout,
        }
    }
    pub fn block(&mut self, block: &'a Block) -> ReturnType<Type> {
        let len = self.stack.len();
        let fun_len = self.funs.len();
        let ret = self.block_inner(block);
        self.stack.drain(len..);
        self.funs.drain(fun_len..);
        ret
    }
    fn block_inner(&mut self, block: &'a Block) -> ReturnType<Type> {
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
                    write_stdout!(self, "{res:?}\n");
                }
                BlockToken::Block(b) => {
                    let res = self.block(b);
                    if i == block.blocks.len() - 1 {
                        return ReturnType::Last(res?);
                    } else if !matches!(res, ReturnType::None) {
                        res?;
                    }
                }
                BlockToken::Function(fn_var, vars, inner) => {
                    self.let_fun(*fn_var, *vars, inner);
                }
                BlockToken::Assignment(var, expr) => {
                    let number = self.expr(expr)?;
                    self.mut_var(*var, number);
                }
                BlockToken::PrintAssignment(var, expr) => {
                    let number = self.expr(expr)?;
                    write_stdout!(self, "{number:?}\n");
                    self.mut_var(*var, number);
                }
                BlockToken::LetAssignment(expr) => {
                    let number = self.expr(expr)?;
                    self.let_var(number);
                }
                BlockToken::PrintLetAssignment(expr) => {
                    let number = self.expr(expr)?;
                    write_stdout!(self, "{number:?}\n");
                    self.let_var(number);
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
                            write_stdout!(self, "{res:?}\n");
                        }
                        ret
                    });
                }
                BlockToken::If(expr, then, ifelse) => {
                    let number = self.expr(expr)?;
                    if let Some(b) = number.bool() {
                        if let Some(res) = if b {
                            Some(self.block(then))
                        } else {
                            ifelse.as_ref().map(|inner| self.block(inner))
                        } {
                            //TODO ifelse.some() may not be saturated
                            if i == block.blocks.len() - 1 && ifelse.is_some() {
                                return ReturnType::Last(res?);
                            } else if !matches!(res, ReturnType::None) {
                                res?;
                            }
                        }
                    } else {
                        write_stdout!(
                            self,
                            "if statement expects a unitless real number, of value 0 or 1\n"
                        );
                    }
                }
            }
        }
        ReturnType::None
    }
    fn let_fun(&mut self, fn_var: u32, vars: u32, block: &'a Block) {
        if fn_var == self.funs.len().strict_cast::<u32>() {
            self.funs.push((vars, block));
        }
    }
    fn let_var(&mut self, number: Type) {
        self.stack.push(number);
    }
    fn mut_var(&mut self, var: u32, number: Type) {
        self.stack[var.strict_cast::<usize>()] = number;
    }
    fn expr(&mut self, expr: &'a Expression) -> ReturnType<Type> {
        let len = self.stack.len();
        for token in &expr.tokens {
            let number = match token {
                ExpressionToken::Type(num) => num.clone(),
                &ExpressionToken::Function(fun) => {
                    self.fun(fun).map_or_default(ReturnType::Last)?
                }
                &ExpressionToken::Variable(v) => {
                    self.stack[self.stack_start + v.strict_cast::<usize>()].clone()
                }
                ExpressionToken::Block(block) => self.block(block)?,
            };
            self.stack.push(number);
        }
        let res = self.stack.pop();
        debug_assert_eq!(self.stack.len(), len);
        res.map_or_default(ReturnType::Last)
    }
    fn custom_fun(&mut self, fun: u32) -> Option<Type> {
        let (count, block) = self.funs[fun.strict_cast::<usize>()];
        let last = self.stack_start;
        self.stack_start = self.stack.len() - count.strict_cast::<usize>();
        let ret = self.block_inner(block).option()?;
        self.stack
            .drain(self.stack.len().checked_sub(count.strict_cast())?..);
        self.stack_start = last;
        Some(ret)
    }
    fn fun(&mut self, fun: Function) -> Option<Type> {
        if let Function::Custom(cfun) = fun {
            return self.custom_fun(cfun);
        }
        let input_count = fun
            .inputs(|c| self.funs[c.strict_cast::<usize>()].0)
            .strict_cast::<usize>();
        let mut inputs = self
            .stack
            .drain(self.stack.len().checked_sub(input_count)?..);
        macro_rules! next {
            () => {
                inputs.next()?
            };
        }
        macro_rules! next_number {
            () => {
                inputs.next()?.number()?
            };
        }
        match fun {
            Function::Vector(i) => {
                if i == 0 {
                    Some(Type::Number(Number::Matrix(Matrix::default())))
                } else {
                    let first = next!();
                    let number_inputs = inputs.filter_map(|t| match t {
                        Type::Number(n) => Some(n),
                        _ => None,
                    });
                    let mat = match first {
                        Type::Number(Number::Complex(c)) => {
                            let dim = MatrixDimension::new(i, 1);
                            let mut new_iter = iter::once(Number::Complex(c)).chain(number_inputs);
                            Matrix::new_with_try(dim, |_, _| new_iter.next()?.complex())?
                        }
                        Type::Number(Number::Matrix(m)) if m.dimension.height == 1 => {
                            let dim = MatrixDimension::new(m.dimension.width, i);
                            let mut new_iter = iter::once(Number::Matrix(m)).chain(number_inputs);
                            Matrix::from_rows_try(dim, |_, _| new_iter.next()?.matrix())?
                        }
                        Type::Number(Number::Matrix(m)) if m.dimension.width == 1 => {
                            let dim = MatrixDimension::new(i, m.dimension.height);
                            let mut new_iter = iter::once(Number::Matrix(m)).chain(number_inputs);
                            Matrix::from_cols_try(dim, |_, _| new_iter.next()?.matrix())?
                        }
                        _ => return None,
                    };
                    Some(Type::Number(Number::Matrix(mat)))
                }
            }
            Function::List(i) => {
                if i == 0 {
                    Some(Type::Number(Number::List(Vec::new())))
                } else {
                    let vec = inputs.collect();
                    Some(Type::List(vec))
                }
            }
            Function::Matrix(i, j) => {
                if i == 0 || j == 0 {
                    Some(Type::Number(Number::Matrix(Matrix::default())))
                } else {
                    let mut number_inputs = inputs.filter_map(|t| match t {
                        Type::Number(n) => Some(n),
                        _ => None,
                    });
                    let dim = MatrixDimension::new(i, j);
                    let mat = Matrix::new_with_try(dim, |_, _| number_inputs.next()?.complex())?;
                    Some(Type::Number(Number::Matrix(mat)))
                }
            }
            Function::AssertEq => {
                let a = next!();
                let b = next!();
                if a != b {
                    write_stdout!(self, "{a:?} does not equal {b:?}\n");
                }
                None
            }
            Function::Operator(o) => match o {
                Operator::Add => (next_number!() + next_number!()).map(Type::Number),
                Operator::Sub => (next_number!() - next_number!()).map(Type::Number),
                Operator::Mul | Operator::MulInner => {
                    (next_number!() * next_number!()).map(Type::Number)
                }
                Operator::Div => (next_number!() / next_number!()).map(Type::Number),
                Operator::Pow => next_number!().pow(&next_number!()).map(Type::Number),
                Operator::Root => next_number!().root(&next_number!()).map(Type::Number),
                Operator::Identity => Some(Type::Number(next_number!())),
                Operator::Negate => Some(Type::Number(-next_number!())),
                Operator::Transpose => next_number!().transpose().map(Type::Number),
                Operator::Factorial => todo!(),
                Operator::SubFactorial => todo!(),
                Operator::Mod => todo!(),
                Operator::Less
                | Operator::LessEq
                | Operator::Greater
                | Operator::GreaterEq
                | Operator::Eq
                | Operator::NotEq => Some(Type::Bool(match o {
                    Operator::Eq => next_number!() == next_number!(),
                    Operator::NotEq => next_number!() != next_number!(),
                    Operator::Less | Operator::LessEq | Operator::Greater | Operator::GreaterEq
                        if let Some(a) = next_number!().complex()
                            && let Some(b) = next_number!().complex()
                            && a.number.im.is_zero()
                            && b.number.im.is_zero()
                            && a.units == b.units =>
                    {
                        match o {
                            Operator::Less => a.number.re < b.number.re,
                            Operator::LessEq => a.number.re <= b.number.re,
                            Operator::Greater => a.number.re > b.number.re,
                            Operator::GreaterEq => a.number.re >= b.number.re,
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                })),
                Operator::And => todo!(),
                Operator::Or => todo!(),
            },
            Function::ComponentWise(o) => match o {
                Operator::Add => next_number!()
                    .component_wise(&next_number!(), Add::add)
                    .map(Type::Number),
                Operator::Sub => next_number!()
                    .component_wise(&next_number!(), Sub::sub)
                    .map(Type::Number),
                Operator::Mul | Operator::MulInner => next_number!()
                    .component_wise(&next_number!(), Mul::mul)
                    .map(Type::Number),
                Operator::Div => next_number!()
                    .component_wise(&next_number!(), Div::div)
                    .map(Type::Number),
                Operator::Pow => next_number!()
                    .component_wise(&next_number!(), Pow::pow)
                    .map(Type::Number),
                Operator::Root => next_number!()
                    .component_wise(&next_number!(), Root::root)
                    .map(Type::Number),
                Operator::Factorial => todo!(),
                Operator::SubFactorial => todo!(),
                Operator::Mod => todo!(),
                Operator::Less => todo!(),
                Operator::LessEq => todo!(),
                Operator::Greater => todo!(),
                Operator::GreaterEq => todo!(),
                Operator::Eq => todo!(),
                Operator::NotEq => todo!(),
                Operator::And => todo!(),
                Operator::Or => todo!(),
                Operator::Negate => Some(Type::Number(-next_number!())),
                Operator::Identity | Operator::Transpose => Some(Type::Number(next_number!())),
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
