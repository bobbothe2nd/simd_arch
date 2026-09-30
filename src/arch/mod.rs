/// vector of 4 `f32` on arch `x86_64` with feature `sse`
#[cfg(target_feature = "sse")]
pub type f32x4 = sse::f32x4;

/// vector of 2 `f64` on arch `x86_64` with feature `sse2`
#[cfg(target_feature = "sse2")]
pub type f64x2 = sse2::f64x2;

/// vector of 16 `u8` on arch `x86_64` with feature `sse2`
#[cfg(target_feature = "sse2")]
pub type u8x16 = sse2::u8x16;

/// vector of 16 `i8` on arch `x86_64` with feature `sse2`
#[cfg(target_feature = "sse2")]
pub type i8x16 = sse2::i8x16;

/// vector of 8 `u16` on arch `x86_64` with feature `sse2`
#[cfg(target_feature = "sse2")]
pub type u16x8 = sse2::u16x8;

/// vector of 8 `i32` on arch `x86_64` with feature `sse2`
#[cfg(target_feature = "sse2")]
pub type i16x8 = sse2::i16x8;

/// vector of 4 `u32` on arch `x86_64` with feature `sse2`
#[cfg(target_feature = "sse2")]
pub type u32x4 = sse2::u32x4;

/// vector of 4 `i64` on arch `x86_64` with feature `sse2`
#[cfg(target_feature = "sse2")]
pub type i32x4 = sse2::i32x4;

/// vector of 2 `u64` on arch `x86_64` with feature `sse2`
#[cfg(target_feature = "sse2")]
pub type u64x2 = sse2::u64x2;

/// vector of 2 `i64` on arch `x86_64` with feature `sse2`
#[cfg(target_feature = "sse2")]
pub type i64x2 = sse2::i64x2;

/// vector of 8 `f32` on arch `x86_64` with feature `avx`
#[cfg(target_feature = "avx")]
pub type f32x8 = avx::f32x8;

/// vector of 4 `f64` on arch `x86_64` with feature `avx`
#[cfg(target_feature = "avx")]
pub type f64x4 = avx::f64x4;

/// vector of 32 `u8` on arch `x86_64` with feature `avx2`
#[cfg(target_feature = "avx2")]
pub type u8x32 = avx2::u8x32;

/// vector of 32 `i8` on arch `x86_64` with feature `avx2`
#[cfg(target_feature = "avx2")]
pub type i8x32 = avx2::i8x32;

/// vector of 16 `u16` on arch `x86_64` with feature `avx2`
#[cfg(target_feature = "avx2")]
pub type u16x16 = avx2::u16x16;

/// vector of 16 `i16` on arch `x86_64` with feature `avx2`
#[cfg(target_feature = "avx2")]
pub type i16x16 = avx2::i16x16;

/// vector of 8 `u32` on arch `x86_64` with feature `avx2`
#[cfg(target_feature = "avx2")]
pub type u32x8 = avx2::u32x8;

/// vector of 8 `i32` on arch `x86_64` with feature `avx2`
#[cfg(target_feature = "avx2")]
pub type i32x8 = avx2::i32x8;

/// vector of 4 `u64` on arch `x86_64` with feature `avx2`
#[cfg(target_feature = "avx2")]
pub type u64x4 = avx2::u64x4;

/// vector of 4 `i64` on arch `x86_64` with feature `avx2`
#[cfg(target_feature = "avx2")]
pub type i64x4 = avx2::i64x4;

#[allow(unused_macros)]
macro_rules! impl_float {
    ($reg:ty => $lane:ty) => {
        impl $reg {
            impl_float!(const MIN: <$reg, $lane>);
            impl_float!(const MAX: <$reg, $lane>);
            impl_float!(const NAN: <$reg, $lane>);
        }
    };

    (const $name:ident: <$reg:ty, $lane:ty>) => {
        pub const $name: $reg = <$reg>::splat_const(<$lane>::$name);
    }
}

#[allow(unused_macros)]
macro_rules! impl_simd {
    (
        $arch:ident = $reg:ident,
        repr = $repr:ident,
        lane = $lane:ty,
        lanes = $lanes:tt,
        width = $width:tt,
        set1 = $set1_intr:ident,
        load = $load_intr:ident,
        loadu = $loadu_intr:ident,
        storeu = $storeu_intr:ident,
        $(+ $([$add_feature:tt])? $add_intr:ident,)?
        $(+s $([$adds_feature:tt])? $adds_intr:ident,)?
        $(* $([$mul_feature:tt])? $mul_intr:ident,)?
        $(- $([$sub_feature:tt])? $sub_intr:ident,)?
        $(-s $([$subs_feature:tt])? $subs_intr:ident,)?
        $(/ $([$div_feature:tt])? $div_intr:ident,)?
        $(1/ $([$($rcp_feature:tt),*])? $rcp_intr:ident,)?
        $(1/~14 $([$($rcp14_feature:tt),*])? $rcp14_intr:ident,)?
        $(*+ $([$fma_feature:tt])? $fma_intr:ident,)?
        $(-*+ $([$fnma_feature:tt])? $fnma_intr:ident,)?
        $(*+- $([$fmas_feature:tt])? $fmas_intr:ident,)?
        $(*- $([$fms_feature:tt])? $fms_intr:ident,)?
        $(-*- $([$fnms_feature:tt])? $fnms_intr:ident,)?
        $(*-+ $([$fmsa_feature:tt])? $fmsa_intr:ident,)?
        $(+- $([$addsub_feature:tt])? $addsub_intr:ident,)?
        $(& $and_intr:ident,)?
        $(& !$andnot_intr:ident,)?
        $(| $or_intr:ident,)?
        $(^ $xor_intr:ident,)?
        $(rnd $([$($rnd_feature:tt),*])? $rnd_intr:ident(
            nearest=$rnd_nearest:ident,
            floor=$rnd_floor:ident,
            ceil=$rnd_ceil:ident,
            trunc=$rnd_trunc:ident$(, exc=$rnd_exc:ident)?,
        ),)?
        $(abs $([$abs_feature:tt])? $abs_intr:ident,)?
        $(absdif $([$absdif_feature:tt])? $absdif_intr:ident,)?
        $(popcnt $([$popcnt_feature:tt])? $popcnt_intr:ident,)?
        $(hadd $([$hadd_feature:tt])? $hadd_intr:ident,)?
        $(hsub $([$hsub_feature:tt])? $hsub_intr:ident,)?
        $(hadds $([$hadds_feature:tt])? $hadds_intr:ident,)?
        $(hsubs $([$hsubs_feature:tt])? $hsubs_intr:ident,)?
        $(dp $([$dp_feature:tt])? $dp_intr:ident($dp_imm8:expr),)?
    ) => {
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct $reg(::core::arch::$arch::$repr);

        unsafe impl briny::traits::StableLayout for $reg {}
        unsafe impl briny::traits::Pod for $reg {}

        impl $reg {
            pub const LANES: usize = $lanes;
            pub const WIDTH: usize = $width;

            #[inline(always)]
            pub fn splat(val: $lane) -> Self {
                Self(unsafe { ::core::arch::$arch::$set1_intr(::briny::raw::cast::reinterpret(val)) })
            }

            #[inline(always)]
            pub const fn splat_const(val: $lane) -> Self {
                Self(::briny::raw::cast::reinterpret([val; $lanes]))
            }

            #[inline(always)]
            pub fn new(slice: &[$lane; $lanes]) -> Self {
                Self(unsafe { ::core::arch::$arch::$loadu_intr(slice.as_ptr().cast()) })
            }

            #[inline(always)]
            pub fn copy(self, ptr: &mut [$lane; $lanes]) {
                unsafe { ::core::arch::$arch::$storeu_intr(ptr.as_mut_ptr().cast(), self.0); }
            }

            #[inline(always)]
            pub fn read(self) -> [$lane; $lanes] {
                let mut out = ::core::mem::MaybeUninit::<[$lane; $lanes]>::uninit();
                unsafe { ::core::arch::$arch::$storeu_intr(out.as_mut_ptr().cast(), self.0); }
                unsafe { out.assume_init() }
            }

            #[inline(always)]
            pub fn from_raw(repr: ::core::arch::x86_64::$repr) -> Self {
                Self(repr)
            }

            #[inline(always)]
            pub fn into_raw(self) -> ::core::arch::x86_64::$repr {
                self.0
            }

            #[inline(always)]
            pub unsafe fn load(ptr: *const $lane) -> Self {
                Self(unsafe { ::core::arch::$arch::$load_intr(ptr.cast()) })
            }

            #[inline(always)]
            pub unsafe fn loadu(ptr: *const $lane) -> Self {
                Self(unsafe { ::core::arch::$arch::$loadu_intr(ptr.cast()) })
            }

            #[inline(always)]
            pub unsafe fn storeu(self, ptr: *mut $lane) {
                unsafe { ::core::arch::$arch::$storeu_intr(ptr.cast(), self.0); }
            }
        }

        $(
            $(#[cfg(target_feature = $add_feature)])?
            impl ::core::ops::Add for $reg {
                type Output = Self;

                #[inline(always)]
                fn add(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$add_intr(self.0, rhs.0))
                    }
                }
            }

            $(#[cfg(target_feature = $add_feature)])?
            impl ::core::ops::AddAssign for $reg {
                #[inline(always)]
                fn add_assign(&mut self, rhs: Self) {
                    self.0 = unsafe {
                        ::core::arch::$arch::$add_intr(self.0, rhs.0)
                    };
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $adds_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn saturating_add(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$adds_intr(self.0, rhs.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $mul_feature)])?
            impl ::core::ops::Mul for $reg {
                type Output = Self;

                #[inline(always)]
                fn mul(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$mul_intr(self.0, rhs.0))
                    }
                }
            }

            $(#[cfg(target_feature = $mul_feature)])?
            impl ::core::ops::MulAssign for $reg {
                #[inline(always)]
                fn mul_assign(&mut self, rhs: Self) {
                    self.0 = unsafe {
                        ::core::arch::$arch::$mul_intr(self.0, rhs.0)
                    };
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $sub_feature)])?
            impl ::core::ops::Sub for $reg {
                type Output = Self;

                #[inline(always)]
                fn sub(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$sub_intr(self.0, rhs.0))
                    }
                }
            }

            $(#[cfg(target_feature = $add_feature)])?
            impl ::core::ops::SubAssign for $reg {
                #[inline(always)]
                fn sub_assign(&mut self, rhs: Self) {
                    self.0 = unsafe {
                        ::core::arch::$arch::$sub_intr(self.0, rhs.0)
                    };
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $subs_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn saturating_sub(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$subs_intr(self.0, rhs.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $div_feature)])?
            impl ::core::ops::Div for $reg {
                type Output = Self;

                #[inline(always)]
                fn div(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$div_intr(self.0, rhs.0))
                    }
                }
            }

            $(#[cfg(target_feature = $div_feature)])?
            impl ::core::ops::DivAssign for $reg {
                #[inline(always)]
                fn div_assign(&mut self, rhs: Self) {
                    self.0 = unsafe {
                        ::core::arch::$arch::$div_intr(self.0, rhs.0)
                    };
                }
            }
        )?

        $(
            $(#[cfg(all($(target_feature = $rcp_feature,)*))])?
            impl $reg {
                /// Reciprocal with precision of `1.5^-12`
                #[inline(always)]
                pub fn rcp(self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$rcp_intr(self.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(all($(target_feature = $rcp14_feature,)*))])?
            impl $reg {
                /// Reciprocal with precision of `2*-14`
                #[inline(always)]
                fn rcp14(self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$rcp14_intr(self.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $fma_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn mul_add(self, b: Self, c: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$fma_intr(self.0, b.0, c.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $fms_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn mul_sub(self, b: Self, c: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$fms_intr(self.0, b.0, c.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $fnms_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn neg_mul_sub(self, b: Self, c: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$fnms_intr(self.0, b.0, c.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $fmas_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn mul_add_sub(self, b: Self, c: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$fmas_intr(self.0, b.0, c.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $fnma_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn neg_mul_add(self, b: Self, c: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$fnma_intr(self.0, b.0, c.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $fmsa_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn mul_sub_add(self, b: Self, c: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$fmsa_intr(self.0, b.0, c.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $addsub_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn add_sub(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$addsub_intr(self.0, rhs.0))
                    }
                }
            }
        )?

        $(
            impl ::core::ops::BitAnd for $reg {
                type Output = Self;

                #[inline(always)]
                fn bitand(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$and_intr(self.0, rhs.0))
                    }
                }
            }
        )?

        $(
            impl $reg {
                #[inline(always)]
                pub fn and_not(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$andnot_intr(self.0, rhs.0))
                    }
                }
            }
        )?

        $(
            impl ::core::ops::BitOr for $reg {
                type Output = Self;

                #[inline(always)]
                fn bitor(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$or_intr(self.0, rhs.0))
                    }
                }
            }
        )?

        $(
            impl ::core::ops::BitXor for $reg {
                type Output = Self;

                #[inline(always)]
                fn bitxor(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$xor_intr(self.0, rhs.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(all($(target_feature = $rnd_feature, )*))])?
            impl $reg {
                #[inline(always)]
                pub fn rnd(self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$rnd_intr::<{ ::core::arch::$arch::$rnd_nearest $(| ::core::arch::$arch::$rnd_exc)? }>(self.0))
                    }
                }

                #[inline(always)]
                pub fn floor(self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$rnd_intr::<{ ::core::arch::$arch::$rnd_floor $(| ::core::arch::$arch::$rnd_exc)? }>(self.0))
                    }
                }

                #[inline(always)]
                pub fn ceil(self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$rnd_intr::<{ ::core::arch::$arch::$rnd_ceil $(| ::core::arch::$arch::$rnd_exc)? }>(self.0))
                    }
                }

                #[inline(always)]
                pub fn trunc(self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$rnd_intr::<{ ::core::arch::$arch::$rnd_trunc $(| ::core::arch::$arch::$rnd_exc)? }>(self.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $abs_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn abs(self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$abs_intr(self.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $absdif_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn abs_dif(self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$absdif_intr(self.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $popcnt_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn count_ones(self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$popcnt_intr(self.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $hadd_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn horizontal_add(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$hadd_intr(self.0, rhs.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $hsub_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn horizontal_sub(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$hsub_intr(self.0, rhs.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $hadds_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn horizontal_saturating_add(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$hadds_intr(self.0, rhs.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $hsubs_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn horizontal_saturating_sub(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$hsubs_intr(self.0, rhs.0))
                    }
                }
            }
        )?

        $(
            $(#[cfg(target_feature = $dp_feature)])?
            impl $reg {
                #[inline(always)]
                pub fn dot_product(self, rhs: Self) -> Self {
                    unsafe {
                        Self(::core::arch::$arch::$dp_intr::<{ $dp_imm8 }>(self.0, rhs.0))
                    }
                }
            }
        )?
    };
}

#[cfg(target_feature = "avx")]
mod avx;

#[cfg(target_feature = "avx2")]
mod avx2;

#[cfg(target_feature = "sse")]
mod sse;

#[cfg(target_feature = "sse2")]
mod sse2;
