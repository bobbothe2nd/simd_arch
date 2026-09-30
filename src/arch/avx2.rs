impl_simd!(
    x86_64 = u8x32,
    repr = __m256i,
    lane = u8,
    lanes = 32,
    width = 32,

    set1 = _mm256_set1_epi8,
    load = _mm256_load_si256,
    loadu = _mm256_loadu_si256,
    storeu = _mm256_storeu_si256,

    + _mm256_adds_epu8,
    +s _mm256_adds_epu8,
    - _mm256_subs_epu8,
    -s _mm256_subs_epu8,
    & _mm256_and_si256,
    &! _mm256_andnot_si256,
    | _mm256_or_si256,
    ^ _mm256_xor_si256,

    popcnt _mm256_popcnt_epi8,
);

impl_simd!(
    x86_64 = i8x32,
    repr = __m256i,
    lane = i8,
    lanes = 32,
    width = 32,

    set1 = _mm256_set1_epi8,
    load = _mm256_load_si256,
    loadu = _mm256_loadu_si256,
    storeu = _mm256_storeu_si256,

    + _mm256_adds_epi8,
    +s _mm256_adds_epi8,
    - _mm256_subs_epi8,
    -s _mm256_subs_epi8,
    & _mm256_and_si256,
    &! _mm256_andnot_si256,
    | _mm256_or_si256,
    ^ _mm256_xor_si256,

    abs _mm256_abs_epi8,
    popcnt _mm256_popcnt_epi8,
);

impl_simd!(
    x86_64 = u16x16,
    repr = __m256i,
    lane = u16,
    lanes = 16,
    width = 32,

    set1 = _mm256_set1_epi16,
    load = _mm256_load_si256,
    loadu = _mm256_loadu_si256,
    storeu = _mm256_storeu_si256,

    + _mm256_adds_epu16,
    +s _mm256_adds_epu16,
    - _mm256_subs_epu16,
    -s _mm256_subs_epu16,
    & _mm256_and_si256,
    &! _mm256_andnot_si256,
    | _mm256_or_si256,
    ^ _mm256_xor_si256,

    popcnt _mm256_popcnt_epi16,
);

impl_simd!(
    x86_64 = i16x16,
    repr = __m256i,
    lane = i16,
    lanes = 16,
    width = 32,

    set1 = _mm256_set1_epi16,
    load = _mm256_load_si256,
    loadu = _mm256_loadu_si256,
    storeu = _mm256_storeu_si256,

    + _mm256_add_epi16,
    +s _mm256_adds_epi16,
    - _mm256_sub_epi16,
    -s _mm256_subs_epi16,
    & _mm256_and_si256,
    &! _mm256_andnot_si256,
    | _mm256_or_si256,
    ^ _mm256_xor_si256,

    abs _mm256_abs_epi16,
    popcnt _mm256_popcnt_epi16,
    hadd _mm256_hadd_epi16,
    hsub _mm256_hsub_epi16,
    hadds _mm256_hadds_epi16,
    hsubs _mm256_hsubs_epi16,
);

impl_simd!(
    x86_64 = u32x8,
    repr = __m256i,
    lane = u32,
    lanes = 8,
    width = 32,

    set1 = _mm256_set1_epi32,
    load = _mm256_load_si256,
    loadu = _mm256_loadu_si256,
    storeu = _mm256_storeu_si256,

    & _mm256_and_si256,
    &! _mm256_andnot_si256,
    | _mm256_or_si256,
    ^ _mm256_xor_si256,

    popcnt _mm256_popcnt_epi32,
);

impl_simd!(
    x86_64 = i32x8,
    repr = __m256i,
    lane = i32,
    lanes = 8,
    width = 32,

    set1 = _mm256_set1_epi32,
    load = _mm256_load_si256,
    loadu = _mm256_loadu_si256,
    storeu = _mm256_storeu_si256,

    + _mm256_add_epi32,
    * _mm256_mullo_epi32,
    - _mm256_sub_epi32,
    & _mm256_and_si256,
    &! _mm256_andnot_si256,
    | _mm256_or_si256,
    ^ _mm256_xor_si256,

    abs _mm256_abs_epi32,
    popcnt _mm256_popcnt_epi32,
    hadd _mm256_hadd_epi32,
    hsub _mm256_hsub_epi32,
);

impl_simd!(
    x86_64 = u64x4,
    repr = __m256i,
    lane = u64,
    lanes = 4,
    width = 32,

    set1 = _mm256_set1_epi64x,
    load = _mm256_load_si256,
    loadu = _mm256_loadu_si256,
    storeu = _mm256_storeu_si256,

    & _mm256_and_si256,
    &! _mm256_andnot_si256,
    | _mm256_or_si256,
    ^ _mm256_xor_si256,

    popcnt _mm256_popcnt_epi64,
);

impl_simd!(
    x86_64 = i64x4,
    repr = __m256i,
    lane = i64,
    lanes = 4,
    width = 32,

    set1 = _mm256_set1_epi64x,
    load = _mm256_load_si256,
    loadu = _mm256_loadu_si256,
    storeu = _mm256_storeu_si256,

    + _mm256_add_epi64,
    * _mm256_mullo_epi64,
    - _mm256_sub_epi32,
    & _mm256_and_si256,
    &! _mm256_andnot_si256,
    | _mm256_or_si256,
    ^ _mm256_xor_si256,

    abs _mm256_abs_epi64,
    popcnt _mm256_popcnt_epi64,
);
