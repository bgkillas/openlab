#![feature(integer_casts)]
#![feature(type_info)]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![feature(gca_min_const_items)]
#![feature(gca_const_items)]
#![feature(generic_const_items)]
#![feature(gca_macroless_args)]
#![feature(option_array_transpose)]
#![feature(portable_simd)]
#![expect(incomplete_features)]
extern crate alloc;
extern crate core;
mod assign_macros;
pub mod float;
pub mod matrix;
pub mod number;
pub mod number_impls;
pub mod rational;
pub mod traits;
pub mod units;
