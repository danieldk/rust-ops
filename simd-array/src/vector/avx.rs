use std::arch::x86_64::{
    __m256, __m256d, __m256i, _mm256_add_pd, _mm256_add_ps, _mm256_and_pd, _mm256_and_ps,
    _mm256_andnot_pd, _mm256_andnot_ps, _mm256_castsi256_pd, _mm256_castsi256_ps, _mm256_cmp_pd,
    _mm256_cmp_ps, _mm256_cvtps_epi32, _mm256_div_pd, _mm256_div_ps, _mm256_floor_pd,
    _mm256_floor_ps, _mm256_load_si256, _mm256_loadu_pd, _mm256_loadu_ps, _mm256_max_pd,
    _mm256_max_ps, _mm256_min_pd, _mm256_min_ps, _mm256_mul_pd, _mm256_mul_ps, _mm256_or_pd,
    _mm256_or_ps, _mm256_set1_epi32, _mm256_set1_epi64x, _mm256_set1_pd, _mm256_set1_ps,
    _mm256_store_pd, _mm256_store_ps, _mm256_storeu_pd, _mm256_storeu_ps, _mm256_sub_pd,
    _mm256_sub_ps, _mm256_xor_pd, _mm256_xor_ps, _CMP_EQ_OQ, _CMP_GT_OQ, _CMP_LT_OQ,
};
use std::mem;
use std::ops::Neg;

use aligned::{Aligned, A32};
use num_traits::{Float, Zero};

use super::scalar::{ScalarVector32, ScalarVector64};
use super::SimdVector;

#[derive(Default)]
pub struct AVXVector32;

impl SimdVector for AVXVector32 {
    type Lower = ScalarVector32;
    type Float = __m256;
    type FloatScalar = f32;
    type FloatScalarArray = Aligned<
        A32,
        [Self::FloatScalar; mem::size_of::<Self::Float>() / mem::size_of::<Self::FloatScalar>()],
    >;
    type Int = __m256i;
    type IntScalar = i32;
    type Mask = __m256;

    #[target_feature(enable = "avx")]
    unsafe fn abs(a: Self::Float) -> Self::Float {
        let mask = _mm256_set1_epi32(0x7fffffff);
        _mm256_and_ps(a, _mm256_castsi256_ps(mask))
    }

    #[target_feature(enable = "avx")]
    unsafe fn add(a: Self::Float, b: Self::Float) -> Self::Float {
        _mm256_add_ps(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn add_scalar(a: Self::Float, b: f32) -> Self::Float {
        let b_simd = _mm256_set1_ps(b);
        _mm256_add_ps(a, b_simd)
    }

    #[target_feature(enable = "avx")]
    unsafe fn bitwise_select(a: Self::Mask, b: Self::Float, c: Self::Float) -> Self::Float {
        // Self::Float::from_bits((a & b.to_bits()) | ((!a) & c.to_bits()))
        let u = _mm256_and_ps(a, b);
        let v = _mm256_andnot_ps(a, c);
        _mm256_or_ps(u, v)
    }

    #[target_feature(enable = "avx")]
    unsafe fn copy_sign(sign_src: Self::Float, dest: Self::Float) -> Self::Float {
        // Negative zero has all bits unset, except the sign bit.
        let sign_bit_mask = Self::splat(Self::FloatScalar::zero().neg());
        Self::bitwise_select(sign_bit_mask, sign_src, dest)
    }

    #[target_feature(enable = "avx")]
    unsafe fn div(a: Self::Float, b: Self::Float) -> Self::Float {
        _mm256_div_ps(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn floor(a: Self::Float) -> Self::Float {
        _mm256_floor_ps(a)
    }

    #[target_feature(enable = "avx")]
    unsafe fn fma(a: Self::Float, b: Self::Float, c: Self::Float) -> Self::Float {
        _mm256_add_ps(_mm256_mul_ps(a, b), c)
    }

    #[target_feature(enable = "avx")]
    unsafe fn eq(a: Self::Float, b: Self::Float) -> Self::Mask {
        _mm256_cmp_ps::<_CMP_EQ_OQ>(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn gt(a: Self::Float, b: Self::Float) -> Self::Mask {
        _mm256_cmp_ps::<_CMP_GT_OQ>(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn lt(a: Self::Float, b: Self::Float) -> Self::Mask {
        _mm256_cmp_ps::<_CMP_LT_OQ>(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn mul(a: Self::Float, b: Self::Float) -> Self::Float {
        _mm256_mul_ps(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn mul_scalar(a: Self::Float, b: f32) -> Self::Float {
        let b_simd = _mm256_set1_ps(b);
        _mm256_mul_ps(a, b_simd)
    }

    #[target_feature(enable = "avx")]
    unsafe fn neg(a: Self::Float) -> Self::Float {
        let neg_zero = _mm256_set1_ps(Self::FloatScalar::neg_zero());
        _mm256_xor_ps(a, neg_zero)
    }

    #[target_feature(enable = "avx")]
    unsafe fn sub(a: Self::Float, b: Self::Float) -> Self::Float {
        _mm256_sub_ps(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn vmax(a: Self::Float, b: Self::Float) -> Self::Float {
        _mm256_max_ps(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn vmin(a: Self::Float, b: Self::Float) -> Self::Float {
        _mm256_min_ps(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn splat(v: f32) -> Self::Float {
        _mm256_set1_ps(v)
    }

    #[target_feature(enable = "avx")]
    unsafe fn reinterpret_float_signed(v: Self::Int) -> Self::Float {
        _mm256_castsi256_ps(v)
    }

    #[target_feature(enable = "avx")]
    unsafe fn to_int(v: Self::Float) -> Self::Int {
        _mm256_cvtps_epi32(v)
    }

    unsafe fn to_float_scalar_array(v: Self::Float) -> Self::FloatScalarArray {
        let mut a: Aligned<A32, _> = Aligned([0f32; 8]);
        _mm256_store_ps(a.as_mut_ptr(), v);
        a
    }

    #[target_feature(enable = "avx")]
    unsafe fn with_load_store(f: &impl Fn(Self::Float) -> Self::Float, a: &mut [f32]) {
        let mut val = _mm256_loadu_ps(a.as_ptr());
        val = f(val);
        _mm256_storeu_ps(a.as_mut_ptr(), val);
    }

    #[target_feature(enable = "avx")]
    unsafe fn apply_elementwise(
        f: impl Fn(Self::Float) -> Self::Float,
        f_rest: impl Fn(&mut [f32]),
        a: &mut [f32],
    ) {
        let v = Self;
        super::apply_elementwise_generic(&v, f, f_rest, a);
    }
}

#[derive(Default)]
pub struct AVXVector64;

impl SimdVector for AVXVector64 {
    type Lower = ScalarVector64;
    type Float = __m256d;
    type FloatScalar = f64;
    type FloatScalarArray = Aligned<
        A32,
        [Self::FloatScalar; mem::size_of::<Self::Float>() / mem::size_of::<Self::FloatScalar>()],
    >;
    type Int = __m256i;
    type IntScalar = i64;
    type Mask = __m256d;

    #[target_feature(enable = "avx")]
    unsafe fn abs(a: Self::Float) -> Self::Float {
        let mask = _mm256_set1_epi64x(0x7fffffffffffffff);
        _mm256_and_pd(a, _mm256_castsi256_pd(mask))
    }

    #[target_feature(enable = "avx")]
    unsafe fn add(a: Self::Float, b: Self::Float) -> Self::Float {
        _mm256_add_pd(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn add_scalar(a: Self::Float, b: f64) -> Self::Float {
        let b_simd = _mm256_set1_pd(b);
        _mm256_add_pd(a, b_simd)
    }

    #[target_feature(enable = "avx")]
    unsafe fn bitwise_select(a: Self::Mask, b: Self::Float, c: Self::Float) -> Self::Float {
        let u = _mm256_and_pd(a, b);
        let v = _mm256_andnot_pd(a, c);
        _mm256_or_pd(u, v)
    }

    #[target_feature(enable = "avx")]
    unsafe fn copy_sign(sign_src: Self::Float, dest: Self::Float) -> Self::Float {
        // Negative zero has all bits unset, except the sign bit.
        let sign_bit_mask = Self::splat(Self::FloatScalar::zero().neg());
        Self::bitwise_select(sign_bit_mask, sign_src, dest)
    }

    #[target_feature(enable = "avx")]
    unsafe fn div(a: Self::Float, b: Self::Float) -> Self::Float {
        _mm256_div_pd(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn floor(a: Self::Float) -> Self::Float {
        _mm256_floor_pd(a)
    }

    #[target_feature(enable = "avx")]
    unsafe fn fma(a: Self::Float, b: Self::Float, c: Self::Float) -> Self::Float {
        _mm256_add_pd(_mm256_mul_pd(a, b), c)
    }

    #[target_feature(enable = "avx")]
    unsafe fn eq(a: Self::Float, b: Self::Float) -> Self::Mask {
        _mm256_cmp_pd::<_CMP_EQ_OQ>(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn gt(a: Self::Float, b: Self::Float) -> Self::Mask {
        _mm256_cmp_pd::<_CMP_GT_OQ>(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn lt(a: Self::Float, b: Self::Float) -> Self::Mask {
        _mm256_cmp_pd::<_CMP_LT_OQ>(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn mul(a: Self::Float, b: Self::Float) -> Self::Float {
        _mm256_mul_pd(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn mul_scalar(a: Self::Float, b: f64) -> Self::Float {
        let b_simd = _mm256_set1_pd(b);
        _mm256_mul_pd(a, b_simd)
    }

    #[target_feature(enable = "avx")]
    unsafe fn neg(a: Self::Float) -> Self::Float {
        let neg_zero = _mm256_set1_pd(Self::FloatScalar::neg_zero());
        _mm256_xor_pd(a, neg_zero)
    }

    #[target_feature(enable = "avx")]
    unsafe fn sub(a: Self::Float, b: Self::Float) -> Self::Float {
        _mm256_sub_pd(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn vmax(a: Self::Float, b: Self::Float) -> Self::Float {
        _mm256_max_pd(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn vmin(a: Self::Float, b: Self::Float) -> Self::Float {
        _mm256_min_pd(a, b)
    }

    #[target_feature(enable = "avx")]
    unsafe fn splat(v: f64) -> Self::Float {
        _mm256_set1_pd(v)
    }

    #[target_feature(enable = "avx")]
    unsafe fn reinterpret_float_signed(v: Self::Int) -> Self::Float {
        _mm256_castsi256_pd(v)
    }

    #[target_feature(enable = "avx")]
    unsafe fn to_int(v: Self::Float) -> Self::Int {
        // Blegh, no instruction for this before AVX-512.
        let mut data_f64: Aligned<A32, _> = Aligned([0f64; 4]);
        _mm256_store_pd(data_f64.as_mut_ptr(), v);
        let data = data_f64.map(|v| v as i64);
        _mm256_load_si256(data.as_ptr().cast())
    }

    #[target_feature(enable = "avx")]
    unsafe fn to_float_scalar_array(v: Self::Float) -> Self::FloatScalarArray {
        let mut a: Aligned<A32, _> = Aligned([0f64; 4]);
        _mm256_store_pd(a.as_mut_ptr(), v);
        a
    }

    #[target_feature(enable = "avx")]
    unsafe fn with_load_store(f: &impl Fn(Self::Float) -> Self::Float, a: &mut [f64]) {
        let mut val = _mm256_loadu_pd(a.as_ptr());
        val = f(val);
        _mm256_storeu_pd(a.as_mut_ptr(), val);
    }

    #[target_feature(enable = "avx")]
    unsafe fn apply_elementwise(
        f: impl Fn(Self::Float) -> Self::Float,
        f_rest: impl Fn(&mut [f64]),
        a: &mut [f64],
    ) {
        let v = Self;
        super::apply_elementwise_generic(&v, f, f_rest, a);
    }
}
