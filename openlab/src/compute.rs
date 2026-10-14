use parser::functions::Function;
use parser::number::traits::pow::{Pow as _, Root as _};
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
    pub fn parse(s: &'a str) -> Result<Self, ParseError> {
        Ok(Self {
            block: Block::parse(s)?,
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
            match token {
                ExpressionToken::Number(num) => self.stack.push(num.clone()),
                &ExpressionToken::Function(fun) => self.fun(fun)?,
                ExpressionToken::Variable(_) => todo!(),
                ExpressionToken::Skip(_) => todo!(),
                ExpressionToken::Block(_) => todo!(),
            }
        }
        let res = self.stack.pop();
        debug_assert_eq!(self.stack, []);
        res
    }
    fn fun(&mut self, fun: Function) -> Option<()> {
        let num = {
            let mut inputs = self
                .stack
                .drain(self.stack.len() - fun.inputs().strict_cast::<usize>()..);
            match fun {
                Function::Add => (inputs.next().unwrap() + inputs.next().unwrap())?,
                Function::Sub => (inputs.next().unwrap() - inputs.next().unwrap())?,
                Function::Mul => (inputs.next().unwrap() * inputs.next().unwrap())?,
                Function::Div => (inputs.next().unwrap() / inputs.next().unwrap())?,
                Function::Pow => inputs.next().unwrap().pow(&inputs.next().unwrap())?,
                Function::Root => inputs.next().unwrap().root(&inputs.next().unwrap())?,
                _ => todo!(),
            }
        };
        self.stack.push(num);
        Some(())
    }
}
