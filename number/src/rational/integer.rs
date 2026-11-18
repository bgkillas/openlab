use core::fmt::Debug;
pub trait IntegerTrait: Sized + Clone + Debug + Copy {
    fn add(self, rhs: Self) -> Option<Self>;
    fn sub(self, rhs: Self) -> Option<Self>;
    fn mul(self, rhs: Self) -> Option<Self>;
    fn div(self, rhs: Self) -> Option<Self>;
}
