impl_simd!(
    x86_64 = f32x4,
    repr = __m128,
    lane = f32,
    lanes = 4,
    width = 16,

    set1 = _mm_set1_ps,
    load = _mm_load_ps,
    loadu = _mm_loadu_ps,
    storeu = _mm_storeu_ps,

    + _mm_add_ps,
    * _mm_mul_ps,
    - _mm_sub_ps,
    / _mm_div_ps,
    1/ ["avx512f", "avx512vl"] _mm_rcp_ps,
    1/~14 ["avx512f", "avx512vl"] _mm_rcp14_ps,
    *+ ["fma"] _mm_fmadd_ps,
    -*+ ["fma"] _mm_fnmadd_ps,
    *+- ["fma"] _mm_fmaddsub_ps,
    *- ["fma"] _mm_fmsub_ps,
    -*- ["fma"] _mm_fnmsub_ps,
    *-+ ["fma"] _mm_fmsubadd_ps,
    +- _mm_addsub_ps,
    & _mm_and_ps,
    &! _mm_andnot_ps,
    | _mm_or_ps,
    ^ _mm_or_ps,

    rnd ["sse4.1"] _mm_round_ps(
        nearest=_MM_FROUND_TO_NEAREST_INT,
        floor=_MM_FROUND_TO_NEG_INF,
        ceil=_MM_FROUND_TO_POS_INF,
        trunc=_MM_FROUND_TO_ZERO,
        exc=_MM_FROUND_NO_EXC,
    ),
    hadd ["sse3"] _mm_hadd_ps,
    hsub ["sse3"] _mm_hsub_ps,
    dp _mm_dp_ps(0b1111_1111),
);

impl_float!(f32x4 => f32);
