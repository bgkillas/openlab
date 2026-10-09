use core::str::FromStr;
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Function {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
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
