use std::simd::Simd;
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Units<T> {
    pub number: T,
    pub units: Simd<f32, 8>,
}
