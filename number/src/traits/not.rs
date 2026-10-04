use core::mem::type_info;
struct Assert<const CONDITION: bool>;
trait True {}
impl True for Assert<true> {}
const NOT<A, B>: bool = type_info::of::<A>() != type_info::of::<B>();
pub trait NotType<T> {}
impl<A, B> NotType<B> for A where Assert<{ NOT::<A, B> }>: True {}
