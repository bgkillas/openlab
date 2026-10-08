#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Function {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}
