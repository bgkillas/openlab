use crate::operators::Operator;
use core::str::FromStr;
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Function {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Root,
    Factorial,
    SubFactorial,
    Negate,
    Mod,
    Abs,
    Identity,
}
impl FromStr for Function {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "add" => Self::Add,
            "sub" => Self::Sub,
            "mul" => Self::Mul,
            "div" => Self::Div,
            "pow" => Self::Pow,
            _ => return Err(()),
        })
    }
}
impl Function {
    pub fn inputs(self) -> u8 {
        match self {
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Pow | Self::Root | Self::Mod => 2,
            Self::Abs | Self::Factorial | Self::SubFactorial | Self::Negate | Self::Identity => 1,
        }
    }
    pub fn inner_vars(self) -> u8 {
        match self {
            Self::Add
            | Self::Sub
            | Self::Mul
            | Self::Div
            | Self::Pow
            | Self::Root
            | Self::Factorial
            | Self::SubFactorial
            | Self::Negate
            | Self::Mod
            | Self::Abs
            | Self::Identity => 0,
        }
    }
}
impl From<Operator> for Function {
    fn from(value: Operator) -> Self {
        match value {
            Operator::Add => Self::Add,
            Operator::Identity => Self::Identity,
            Operator::Sub => Self::Sub,
            Operator::Mul => Self::Mul,
            Operator::Div => Self::Div,
            Operator::Pow => Self::Pow,
            Operator::Root => Self::Root,
            Operator::Factorial => Self::Factorial,
            Operator::SubFactorial => Self::SubFactorial,
            Operator::Negate => Self::Negate,
            Operator::Mod => Self::Mod,
            Operator::Function(f) => f,
            Operator::LeftBracket(_) => unreachable!(),
        }
    }
}
