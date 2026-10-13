use parser::functions::Function;
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
                    debug_assert_eq!(self.inner.stack.len(), 1);
                }
                BlockToken::PrintExpression(expr) => {
                    self.inner.expr(expr);
                    debug_assert_eq!(self.inner.stack.len(), 1);
                    println!("{:?}", self.inner.stack[0]);
                }
                _ => todo!(),
            }
        }
    }
}
impl ComputeInner {
    fn expr(&mut self, expr: &Expression) {
        for token in &expr.tokens {
            match token {
                ExpressionToken::Number(num) => self.stack.push(num.clone()),
                &ExpressionToken::Function(fun) => self.fun(fun),
                ExpressionToken::Variable(_) => todo!(),
                ExpressionToken::Skip(_) => todo!(),
            }
        }
    }
    fn fun(&mut self, fun: Function) {
        match fun {
            Function::Add => {
                let last = self.stack.pop().unwrap();
                self.stack.last_mut().unwrap().add_assign(&last);
            }
            Function::Sub => {
                let last = self.stack.pop().unwrap();
                self.stack.last_mut().unwrap().sub_assign(&last);
            }
            Function::Mul => {
                let last = self.stack.pop().unwrap();
                self.stack.last_mut().unwrap().mul_assign(&last);
            }
            Function::Div => {
                let last = self.stack.pop().unwrap();
                self.stack.last_mut().unwrap().div_assign(&last);
            }
            Function::Pow => {
                let last = self.stack.pop().unwrap();
                self.stack.last_mut().unwrap().pow_assign(&last);
            }
            Function::Root => {
                let last = self.stack.pop().unwrap();
                self.stack.last_mut().unwrap().root_assign(&last);
            }
            _ => todo!(),
        }
    }
}
