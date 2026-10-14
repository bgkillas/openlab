pub trait Pow<Rhs> {
    type Output;
    fn pow(self, rhs: Rhs) -> Self::Output;
}
pub trait Root<Rhs> {
    type Output;
    fn root(self, rhs: Rhs) -> Self::Output;
}
pub trait PowAssign<Rhs> {
    fn pow_assign(&mut self, rhs: Rhs);
}
pub trait RootAssign<Rhs> {
    fn root_assign(&mut self, rhs: Rhs);
}
