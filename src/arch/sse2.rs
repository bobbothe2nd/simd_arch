impl_simd!(
    x86_64 = f64x2,
    repr = __m128d,
    lane = f64,
    lanes = 2,
    width = 16,

    set1 = _mm_set1_pd,
    load = _mm_load_pd,
    loadu = _mm_loadu_pd,
    storeu = _mm_storeu_pd,

    + _mm_add_pd,
    * _mm_mul_pd,
    - _mm_sub_pd,
    / _mm_div_pd,
    1/ ["avx512f", "avx512vl"] _mm_rcp14_pd,
    1/~14 ["avx512f", "avx512vl"] _mm_rcp14_pd,
    *+ ["fma"] _mm_fmadd_pd,
    -*+ ["fma"] _mm_fnmadd_pd,
    *+- ["fma"] _mm_fmaddsub_pd,
    *- ["fma"] _mm_fmsub_pd,
    -*- ["fma"] _mm_fnmsub_pd,
    *-+ ["fma"] _mm_fmsubadd_pd,
    +- _mm_addsub_pd,
    & _mm_and_pd,
    &! _mm_andnot_pd,
    | _mm_or_pd,
    ^ _mm_xor_pd,

    rnd ["sse4.1"] _mm_round_pd(
        nearest=_MM_FROUND_TO_NEAREST_INT,
        floor=_MM_FROUND_TO_NEG_INF,
        ceil=_MM_FROUND_TO_POS_INF,
        trunc=_MM_FROUND_TO_ZERO,
        exc=_MM_FROUND_NO_EXC,
    ),
    hadd ["sse3"] _mm_hadd_pd,
    hsub ["sse3"] _mm_hsub_pd,
    dp _mm_dp_pd(0b1111_1),
);

impl_float!(f64x2 => f64);

impl_simd!(
    x86_64 = u8x16,
    repr = __m128i,
    lane = u8,
    lanes = 16,
    width = 16,

    set1 = _mm_set1_epi8,
    load = _mm_load_si128,
    loadu = _mm_loadu_si128,
    storeu = _mm_storeu_si128,

    + _mm_adds_epu8,
    +s _mm_adds_epu8,
    - _mm_subs_epu8,
    -s _mm_subs_epu8,
    & _mm_and_si128,
    &! _mm_andnot_si128,
    | _mm_or_si128,
    ^ _mm_xor_si128,

    popcnt _mm_popcnt_epi8,
);

impl_simd!(
    x86_64 = i8x16,
    repr = __m128i,
    lane = i8,
    lanes = 16,
    width = 16,

    set1 = _mm_set1_epi8,
    load = _mm_load_si128,
    loadu = _mm_loadu_si128,
    storeu = _mm_storeu_si128,

    + _mm_adds_epi8,
    +s _mm_adds_epi8,
    - _mm_subs_epi8,
    -s _mm_subs_epi8,
    & _mm_and_si128,
    &! _mm_andnot_si128,
    | _mm_or_si128,
    ^ _mm_xor_si128,

    abs _mm_abs_epi8,
    popcnt _mm_popcnt_epi8,
);

impl_simd!(
    x86_64 = u16x8,
    repr = __m128i,
    lane = u16,
    lanes = 8,
    width = 16,

    set1 = _mm_set1_epi16,
    load = _mm_load_si128,
    loadu = _mm_loadu_si128,
    storeu = _mm_storeu_si128,

    + _mm_adds_epu16,
    +s _mm_adds_epu16,
    - _mm_subs_epu16,
    -s _mm_subs_epu16,
    & _mm_and_si128,
    &! _mm_andnot_si128,
    | _mm_or_si128,
    ^ _mm_xor_si128,

    popcnt _mm_popcnt_epi16,
);

impl_simd!(
    x86_64 = i16x8,
    repr = __m128i,
    lane = i16,
    lanes = 8,
    width = 16,

    set1 = _mm_set1_epi16,
    load = _mm_load_si128,
    loadu = _mm_loadu_si128,
    storeu = _mm_storeu_si128,

    + _mm_add_epi16,
    +s _mm_adds_epi16,
    - _mm_sub_epi16,
    -s _mm_subs_epi16,
    & _mm_and_si128,
    &! _mm_andnot_si128,
    | _mm_or_si128,
    ^ _mm_xor_si128,

    abs _mm_abs_epi16,
    popcnt _mm_popcnt_epi16,
    hadd ["ssse3"] _mm_hadd_epi16,
    hsub ["ssse3"] _mm_hsub_epi16,
    hadds ["ssse3"] _mm_hadds_epi16,
    hsubs ["ssse3"] _mm_hsubs_epi16,
);

impl_simd!(
    x86_64 = u32x4,
    repr = __m128i,
    lane = u32,
    lanes = 4,
    width = 16,

    set1 = _mm_set1_epi32,
    load = _mm_load_si128,
    loadu = _mm_loadu_si128,
    storeu = _mm_storeu_si128,

    & _mm_and_si128,
    &! _mm_andnot_si128,
    | _mm_or_si128,
    ^ _mm_xor_si128,

    popcnt _mm_popcnt_epi32,
);

impl_simd!(
    x86_64 = i32x4,
    repr = __m128i,
    lane = i32,
    lanes = 4,
    width = 16,

    set1 = _mm_set1_epi32,
    load = _mm_load_si128,
    loadu = _mm_loadu_si128,
    storeu = _mm_storeu_si128,

    + _mm_add_epi32,
    * _mm_mullo_epi32,
    - _mm_sub_epi32,
    & _mm_and_si128,
    &! _mm_andnot_si128,
    | _mm_or_si128,
    ^ _mm_xor_si128,

    abs _mm_abs_epi32,
    popcnt _mm_popcnt_epi32,
    hadd ["ssse3"] _mm_hadd_epi32,
    hsub ["ssse3"] _mm_hsub_epi32,
);

impl_simd!(
    x86_64 = u64x2,
    repr = __m128i,
    lane = u64,
    lanes = 2,
    width = 16,

    set1 = _mm_set1_epi64x,
    load = _mm_load_si128,
    loadu = _mm_loadu_si128,
    storeu = _mm_storeu_si128,

    & _mm_and_si128,
    &! _mm_andnot_si128,
    | _mm_or_si128,
    ^ _mm_xor_si128,

    popcnt _mm_popcnt_epi64,
);

impl_simd!(
    x86_64 = i64x2,
    repr = __m128i,
    lane = i64,
    lanes = 2,
    width = 16,

    set1 = _mm_set1_epi64x,
    load = _mm_load_si128,
    loadu = _mm_loadu_si128,
    storeu = _mm_storeu_si128,

    + _mm_add_epi64,
    * _mm_mullo_epi64,
    - _mm_sub_epi32,
    & _mm_and_si128,
    &! _mm_andnot_si128,
    | _mm_or_si128,
    ^ _mm_xor_si128,

    abs _mm_abs_epi64,
    popcnt _mm_popcnt_epi64,
);
