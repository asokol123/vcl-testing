#![feature(stdsimd)]

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub mod vec32b;
