use number::float::complex::Complex;
use crate::functions::Function;
pub type NumberInner = Complex<f64>;
pub type Number = number::number::Number<NumberInner>;
pub struct Expression {
    pub tokens: Vec<ExpressionToken>,
}
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Variable {
    Variable(u32),
    Function(u32),
    Global(u32),
}
pub enum ExpressionToken {
    Number(Number),
    Variable(Variable),
    Function(Function),
    Skip(u32),
}
pub struct Block {
    pub tokens: Vec<BlockToken>,
}
pub enum BlockToken {
    Expression(Expression),
    PrintExpression(Expression),
    Assignment(Variable, Expression),
    PrintAssignment(Variable, Expression),
    Return(Vec<Variable>),
    PrintReturn(Vec<Variable>),
}