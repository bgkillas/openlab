#[macro_export]
macro_rules! assign_impl {
    ($tr:ident, $fun:ident, $lower:ident, $lower_fun:ident, $t:ty, $o:ty) => {
        impl $tr<$o> for $t {
            fn $fun(&mut self, rhs: $o) {
                replace_with_or_abort(self, |lhs| <Self as $lower<$o>>::$lower_fun(lhs, rhs));
            }
        }
    };
    ($tr:ident, $fun:ident, $lower:ident, $lower_fun:ident, $t:ty, $o:ty, $($rest:tt)*) => {
        impl<$($rest)*> $tr<$o> for $t {
            fn $fun(&mut self, rhs: $o) {
                replace_with_or_abort(self, |lhs| <Self as $lower<$o>>::$lower_fun(lhs, rhs));
            }
        }
    };
}
#[macro_export]
macro_rules! assign_each {
    ($t:ty, $o:ty) => {
        assign_impl!(AddAssign, add_assign, Add, add, $t, $o);
        assign_impl!(SubAssign, sub_assign, Sub, sub, $t, $o);
        assign_impl!(MulAssign, mul_assign, Mul, mul, $t, $o);
        assign_impl!(DivAssign, div_assign, Div, div, $t, $o);
    };
    ($t:ty, $o:ty, $($rest:tt)*) => {
        assign_impl!(AddAssign, add_assign, Add, add, $t, $o, $($rest)*);
        assign_impl!(SubAssign, sub_assign, Sub, sub, $t, $o, $($rest)*);
        assign_impl!(MulAssign, mul_assign, Mul, mul, $t, $o, $($rest)*);
        assign_impl!(DivAssign, div_assign, Div, div, $t, $o, $($rest)*);
    };
}
