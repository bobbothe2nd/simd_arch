impl_simd!(
    x86_64 = f32x8,
    repr = __m256,
    lane = f32,
    lanes = 8,
    width = 32,

    set1 = _mm256_set1_ps,
    load = _mm256_load_ps,
    loadu = _mm256_loadu_ps,
    storeu = _mm256_storeu_ps,

    + _mm256_add_ps,
    * _mm256_mul_ps,
    - _mm256_sub_ps,
    / _mm256_div_ps,
    1/ ["avx512f", "avx512vl"] _mm256_rcp_ps,
    1/~14 ["avx512f", "avx512vl"] _mm256_rcp14_ps,
    *+ ["fma"] _mm256_fmadd_ps,
    -*+ ["fma"] _mm256_fnmadd_ps,
    *+- ["fma"] _mm256_fmaddsub_ps,
    *- ["fma"] _mm256_fmsub_ps,
    -*- ["fma"] _mm256_fnmsub_ps,
    *-+ ["fma"] _mm256_fmsubadd_ps,
    +- _mm256_addsub_ps,
    & _mm256_and_ps,
    &! _mm256_andnot_ps,
    | _mm256_or_ps,
    ^ _mm256_xor_ps,

    rnd ["avx512f", "avx512vl"] _mm256_roundscale_ps(
        nearest=_MM_FROUND_TO_NEAREST_INT,
        floor=_MM_FROUND_TO_NEG_INF,
        ceil=_MM_FROUND_TO_POS_INF,
        trunc=_MM_FROUND_TO_ZERO,
        exc=_MM_FROUND_NO_EXC,
    ),
    hadd _mm256_hadd_ps,
    hsub _mm256_hsub_ps,
    dp _mm256_dp_ps(0b1111_1111),
);

impl_float!(f32x8 => f32);

impl_simd!(
    x86_64 = f64x4,
    repr = __m256d,
    lane = f64,
    lanes = 4,
    width = 32,

    set1 = _mm256_set1_pd,
    load = _mm256_load_pd,
    loadu = _mm256_loadu_pd,
    storeu = _mm256_storeu_pd,

    + _mm256_add_pd,
    * _mm256_mul_pd,
    - _mm256_sub_pd,
    / _mm256_div_pd,
    1/ ["avx512f", "avx512vl"] _mm256_rcp14_pd,
    1/~14 ["avx512f", "avx512vl"] _mm256_rcp14_pd,
    *+ ["fma"] _mm256_fmadd_pd,
    -*+ ["fma"] _mm256_fnmadd_pd,
    *+- ["fma"] _mm256_fmaddsub_pd,
    *- ["fma"] _mm256_fmsub_pd,
    -*- ["fma"] _mm256_fnmsub_pd,
    *-+ ["fma"] _mm256_fmsubadd_pd,
    +- _mm256_addsub_pd,
    & _mm256_and_pd,
    &! _mm256_andnot_pd,
    | _mm256_or_pd,
    ^ _mm256_xor_pd,

    rnd ["avx512f", "avx512vl"] _mm256_roundscale_pd(
        nearest=_MM_FROUND_TO_NEAREST_INT,
        floor=_MM_FROUND_TO_NEG_INF,
        ceil=_MM_FROUND_TO_POS_INF,
        trunc=_MM_FROUND_TO_ZERO,
        exc=_MM_FROUND_NO_EXC,
    ),
    hadd _mm256_hadd_pd,
    hsub _mm256_hsub_pd,
);

impl_float!(f64x4 => f64);
