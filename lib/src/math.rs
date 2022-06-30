use ::safer_ffi::prelude::*;

#[ffi_export]
fn rs_floor(i: f64) -> f64 {
    i.floor()
}

#[ffi_export]
fn rs_ceil(i: f64) -> f64 {
    i.ceil()
}

#[ffi_export]
fn rs_round(i: f64) -> f64 {
    i.round()
}

#[ffi_export]
fn rs_trunc(i: f64) -> f64 {
    i.trunc()
}

#[ffi_export]
fn rs_fract(i: f64) -> f64 {
    i.fract()
}

#[ffi_export]
fn rs_abs(i: f64) -> f64 {
    i.abs()
}

#[ffi_export]
fn rs_signum(i: f64) -> f64 {
    i.signum()
}

#[ffi_export]
fn rs_copysign(i: f64, sign: f64) -> f64 {
    i.copysign(sign)
}

#[ffi_export]
fn rs_mul_add(i: f64, a: f64, b: f64) -> f64 {
    i.mul_add(a, b)
}

#[ffi_export]
fn rs_div_euclid(i: f64, rhs: f64) -> f64 {
    i.div_euclid(rhs)
}

#[ffi_export]
fn rs_rem_euclid(i: f64, rhs: f64) -> f64 {
    i.rem_euclid(rhs)
}

#[ffi_export]
fn rs_powi(i: f64, n: i32) -> f64 {
    i.powi(n)
}

#[ffi_export]
fn rs_powf(i: f64, n: f64) -> f64 {
    i.powf(n)
}

#[ffi_export]
fn rs_sqrt(i: f64) -> f64 {
    i.sqrt()
}

#[ffi_export]
fn rs_exp(i: f64) -> f64 {
    i.exp()
}

#[ffi_export]
fn rs_exp2(i: f64) -> f64 {
    i.exp2()
}

#[ffi_export]
fn rs_ln(i: f64) -> f64 {
    i.ln()
}

#[ffi_export]
fn rs_log(i: f64, base: f64) -> f64 {
    i.log(base)
}

#[ffi_export]
fn rs_log2(i: f64) -> f64 {
    i.log2()
}

#[ffi_export]
fn rs_log10(i: f64) -> f64 {
    i.log10()
}

#[ffi_export]
fn rs_cbrt(i: f64) -> f64 {
    i.cbrt()
}

#[ffi_export]
fn rs_hypot(i: f64, j: f64) -> f64 {
    i.hypot(j)
}

#[ffi_export]
fn rs_sin(i: f64) -> f64 {
    i.sin()
}

#[ffi_export]
fn rs_cos(i: f64) -> f64 {
    i.cos()
}

#[ffi_export]
fn rs_tan(i: f64) -> f64 {
    i.tan()
}

#[ffi_export]
fn rs_asin(i: f64) -> f64 {
    i.asin()
}

#[ffi_export]
fn rs_acos(i: f64) -> f64 {
    i.acos()
}

#[ffi_export]
fn rs_atan(i: f64) -> f64 {
    i.atan()
}

#[ffi_export]
fn rs_atan2(i: f64, j: f64) -> f64 {
    i.atan2(j)
}

#[ffi_export]
fn rs_exp_m1(i: f64) -> f64 {
    i.exp_m1()
}

#[ffi_export]
fn rs_ln_1p(i: f64) -> f64 {
    i.ln_1p()
}

#[ffi_export]
fn rs_sinh(i: f64) -> f64 {
    i.sinh()
}

#[ffi_export]
fn rs_cosh(i: f64) -> f64 {
    i.cosh()
}

#[ffi_export]
fn rs_tanh(i: f64) -> f64 {
    i.tanh()
}

#[ffi_export]
fn rs_asinh(i: f64) -> f64 {
    i.asinh()
}

#[ffi_export]
fn rs_acosh(i: f64) -> f64 {
    i.acosh()
}

#[ffi_export]
fn rs_atanh(i: f64) -> f64 {
    i.atanh()
}

#[ffi_export]
fn rs_is_nan(i: f64) -> bool {
    i.is_nan()
}

#[ffi_export]
fn rs_is_infinite(i: f64) -> bool {
    i.is_infinite()
}

#[ffi_export]
fn rs_is_finite(i: f64) -> bool {
    i.is_finite()
}

#[ffi_export]
fn rs_is_subnormal(i: f64) -> bool {
    i.is_subnormal()
}

#[ffi_export]
fn rs_is_normal(i: f64) -> bool {
    i.is_normal()
}

#[ffi_export]
fn rs_is_sign_positive(i: f64) -> bool {
    i.is_sign_positive()
}

#[ffi_export]
fn rs_is_sign_negative(i: f64) -> bool {
    i.is_sign_negative()
}

#[ffi_export]
fn rs_recip(i: f64) -> f64 {
    i.recip()
}

#[ffi_export]
fn rs_to_degrees(i: f64) -> f64 {
    i.to_degrees()
}

#[ffi_export]
fn rs_to_radians(i: f64) -> f64 {
    i.to_radians()
}

#[ffi_export]
fn rs_max(i: f64, j: f64) -> f64 {
    i.max(j)
}

#[ffi_export]
fn rs_min(i: f64, j: f64) -> f64 {
    i.min(j)
}

#[derive_ReprC]
#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct ConversionResult {
    result: i32,
    error: Option<safer_ffi::char_p::char_p_boxed>
}

#[ffi_export]
fn rs_to_int(i: f64) -> ConversionResult {
    if i.is_infinite()  {
        ConversionResult { result: 0, error: Some(char_p::new("i is infinite"))}
    }else if i.is_nan() {
        ConversionResult { result: 0, error: Some(char_p::new("i is nan"))} 
    } else if i > std::i32::MAX as f64 {
        ConversionResult { result: 0, error: Some(char_p::new("i is too big"))}
    } else if i < std::i32::MIN as f64 {
        ConversionResult { result: 0, error: Some(char_p::new("i is too small"))}
    } else {
        ConversionResult { result: unsafe { i.to_int_unchecked::<i32>()}, error: None}
    } 
}

#[ffi_export]
fn rs_to_float(i: i32) -> f64 {
    i.into()
}
