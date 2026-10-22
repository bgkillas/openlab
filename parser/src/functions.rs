use crate::operators::{Operator, OperatorToken};
use core::ops::Range;
use core::str::FromStr;
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Function {
    Operator(Operator),
    ComponentWise(Operator),
    Abs,
    Vector(u32),
    Custom(u32),
}
impl FromStr for Function {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "add" => Self::Operator(Operator::Add),
            "sub" => Self::Operator(Operator::Sub),
            "mul" => Self::Operator(Operator::Mul),
            "div" => Self::Operator(Operator::Div),
            "pow" => Self::Operator(Operator::Pow),
            "transpose" => Self::Operator(Operator::Transpose),
            _ => return Err(()),
        })
    }
}
impl Function {
    pub fn input_range(self) -> Range<u32> {
        match self {
            Self::Operator(o) | Self::ComponentWise(o) => o.input_range(),
            Self::Abs => 1..2,
            Self::Vector(_) => 0..u32::MAX,
            Self::Custom(_) => todo!(),
        }
    }
    pub fn inputs(self) -> u32 {
        match self {
            Self::Operator(o) | Self::ComponentWise(o) => o.inputs(),
            Self::Abs => 1,
            Self::Vector(v) => v,
            Self::Custom(_) => todo!(),
        }
    }
    pub fn inner_vars(self) -> u8 {
        match self {
            Self::Operator(o) | Self::ComponentWise(o) => o.inner_vars(),
            Self::Abs | Self::Vector(_) => 0,
            Self::Custom(_) => todo!(),
        }
    }
}
impl From<OperatorToken> for Function {
    fn from(value: OperatorToken) -> Self {
        match value {
            OperatorToken::Function(f) => f,
            OperatorToken::Operator(o) => Self::Operator(o),
            OperatorToken::ComponentWise(o) => Self::ComponentWise(o),
            OperatorToken::LeftBracket(_) => unreachable!(),
        }
    }
}
