//! Most functions are guaranteed to lower to only one SIMD instruction.

#![allow(non_camel_case_types)]
#![no_std]

#[cfg(any(target_feature = "avx", target_feature = "avx2"))]
mod arch;

#[cfg(any(target_feature = "avx", target_feature = "avx2"))]
pub use arch::*;
