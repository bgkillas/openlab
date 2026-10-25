#![feature(portable_simd)]
#![feature(variant_count)]
#![feature(integer_casts)]
#![feature(const_trait_impl)]
#![feature(gca_min_const_items)]
#![feature(gca_const_items)]
#![feature(generic_const_items)]
#![feature(gca_macroless_args)]
#![expect(incomplete_features)]
pub mod functions;
pub mod operators;
pub mod options;
pub mod parser;
pub mod units;
#[cfg(test)]
mod units_test;
pub use lexer;
pub use number;
