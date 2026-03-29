//! The `fast-powi` crate provides fast exponentiation for numeric types.
//!
//! This crate provides a `const` version of `powi` for floats, as well as
//! optimized implementations for `u8` and `i8` exponents for numeric types.
//! Exponentiation is optimized using a table of shortest multiplication chains.
//! `Square` and `Cube` traits are also provided for further optimization of
//! types that can be squared and cubed more efficiently than by repeated
//! multiplication, such as for complex numbers.

use num_bigint::{BigInt, BigUint};
use num_complex::Complex;
use num_rational::Ratio;
use num_traits::Inv;
use std::ops::Mul;

#[macro_use]
mod macros;

/// Trait for squaring `self`.
///
/// This trait provides a squaring function that is at least as efficient 
/// as repeated multiplication.
///
/// # Example
/// ```
/// use fast_powi::Square;
/// use num_complex::Complex;
/// use num_rational::Ratio;
///
/// let a = Complex::new(Ratio::new(3, 5), Ratio::new(4, 5));
/// assert_eq!(a.sq(), a * a);
/// ```
pub trait Square: Mul + Sized {
    /// Returns the square of `self`.
    fn sq(self) -> <Self as Mul>::Output;
}

/// Trait for cubing `self`.
///
/// This trait provides a cubing function that is at least as efficient 
/// as repeated multiplication.
///
/// # Example
/// ```
/// use fast_powi::Cube;
/// use num_complex::Complex;
/// use num_rational::Ratio;
///
/// let a = Complex::new(Ratio::new(3, 5), Ratio::new(4, 5));
/// assert_eq!(a.cb(), a * a * a);
/// ```
pub trait Cube: Mul + Sized {
    /// Returns the cube of `self`.
    fn cb(self) -> <Self as Mul>::Output;
}

/// Trait for raising `self` to the power of a `u8`.
/// 
/// This trait provides a `powu8` function that is faster than what is provided
/// in the Rust standard library or in `num_traits::Pow`. Since `powu8` only
/// accepts a `u8` exponent, the range of exponents is limited to `0..=255`.
///
/// # Example
/// ```
/// use fast_powi::PowU8;
/// use num_complex::Complex;
/// use num_rational::Ratio;
///
/// let a = Complex::new(Ratio::new(3, 5), Ratio::new(4, 5));
/// assert_eq!(a.powu8(5), a * a * a * a * a);
/// ```
pub trait PowU8: Square {
    /// Raises `self` to the power of `exp`.
    fn powu8(self, exp: u8) -> <Self as Mul>::Output;
}

/// Trait for raising `self` to the power of an `i8`.
///
/// This trait provides a `powi8` function that is faster than what is provided
/// in the Rust standard library or in `num_traits::Pow`. Since `powi8` only
/// accepts an `i8` exponent, the range of exponents is limited to `-128..=127`.
/// 
/// # Example
/// ```
/// use fast_powi::PowI8;
/// use num_complex::Complex;
/// use num_rational::Ratio;
/// use num_traits::ConstOne;
///
/// const ONE: Complex<Ratio<i32>> = ConstOne::ONE;
/// let a = Complex::new(Ratio::new(3, 5), Ratio::new(4, 5));
/// assert_eq!(a.powi8(-5), ONE / (a * a * a * a * a));
/// ```
pub trait PowI8: PowU8 {
    /// Raises `self` to the power of `exp`.
    fn powi8(self, exp: i8) -> <Self as Mul>::Output;
}

macro_rules! forward_ref_pow {
    (impl $imp:ident, $method:ident($($arg:ident: $arg_ty:ty),*) for $b:ty) => {
        impl $imp for &$b {
            #[inline]
            fn $method(self $(, $arg : $arg_ty)*) -> $b {
                $imp::$method(*self, $($arg),*)
            }
        }
    };
}

macro_rules! forward_owned_pow {
    (impl $imp:ident, $method:ident($($arg:ident: $arg_ty:ty),*) for $b:ty) => {
        impl $imp for $b {
            #[inline]
            fn $method(self $(, $arg : $arg_ty)*) -> $b {
                $imp::$method(&self, $($arg),*)
            }
        }
    };
}

macro_rules! impl_powu8_for {
    ($t: ty) => {
        forward_owned_pow!(impl Square, sq() for $t);
        forward_owned_pow!(impl Cube, cb() for $t);
        forward_owned_pow!(impl PowU8, powu8(exp: u8) for $t);
        impl Square for &$t {
            #[inline]
            fn sq(self) -> $t {
                self * self
            }
        }
        impl Cube for &$t {
            #[inline]
            fn cb(self) -> $t {
                self * self * self
            }
        }
        impl PowU8 for &$t {
            #[inline]
            fn powu8(self, exp: u8) -> $t {
                static POW: [fn(&$t) -> $t; 256] = pow_array_u8!();
                unsafe { POW.get_unchecked(exp as usize)(self) }
            }
        }
    };
    ($t: ty: Copy) => {
        impl Square for $t {
            #[inline]
            fn sq(self) -> $t {
                self * self
            }
        }
        impl Cube for $t {
            #[inline]
            fn cb(self) -> $t {
                self * self * self
            }
        }
        impl PowU8 for $t {
            #[inline]
            fn powu8(self, exp: u8) -> $t {
                static POW: [fn($t) -> $t; 256] = pow_array_u8!();
                unsafe { POW.get_unchecked(exp as usize)(self) }
            }
        }
        forward_ref_pow!(impl Square, sq() for $t);
        forward_ref_pow!(impl Cube, cb() for $t);
        forward_ref_pow!(impl PowU8, powu8(exp: u8) for $t);
    };
}

impl_powu8_for!(BigInt);
impl_powu8_for!(BigUint);
impl_powu8_for!(f32: Copy);
impl_powu8_for!(f64: Copy);
impl_powu8_for!(i8: Copy);
impl_powu8_for!(i16: Copy);
impl_powu8_for!(i32: Copy);
impl_powu8_for!(i64: Copy);
impl_powu8_for!(u8: Copy);
impl_powu8_for!(u16: Copy);
impl_powu8_for!(u32: Copy);
impl_powu8_for!(u64: Copy);

macro_rules! impl_powi8_for {
    ($t: ty: Copy) => {
        impl PowI8 for $t {
            #[inline]
            fn powi8(self, exp: i8) -> $t {
                static POW: [fn($t) -> $t; 256] = pow_array_i8!();
                unsafe { POW.get_unchecked(exp as u8 as usize)(self) }
            }
        }
        forward_ref_pow!(impl PowI8, powi8(exp: i8) for $t);
    };
    ($t: ty) => {
        forward_owned_pow!(impl PowI8, powi8(exp: i8) for $t);
        impl PowI8 for &$t {
            #[inline]
            fn powi8(self, exp: i8) -> $t {
                static POW: [fn(&$t) -> $t; 256] = pow_array_i8!();
                unsafe { POW.get_unchecked(exp as u8 as usize)(self) }
            }
        }
    };
}

impl_powi8_for!(f32: Copy);
impl_powi8_for!(f64: Copy);

macro_rules! impl_pow_for_ratio {
    ($t: ty: Copy) => {
        impl Square for Ratio<$t> {
            #[inline]
            fn sq(self) -> Ratio<$t> {
                Ratio::new_raw(self.numer().sq(), self.denom().sq())
            }
        }
        impl Cube for Ratio<$t> {
            #[inline]
            fn cb(self) -> Ratio<$t> {
                Ratio::new_raw(self.numer().cb(), self.denom().cb())
            }
        }
        impl PowU8 for Ratio<$t> {
            #[inline]
            fn powu8(self, exp: u8) -> Ratio<$t> {
                Ratio::new_raw(self.numer().powu8(exp), self.denom().powu8(exp))
            }
        }
        forward_ref_pow!(impl Square, sq() for Ratio<$t>);
        forward_ref_pow!(impl Cube, cb() for Ratio<$t>);
        forward_ref_pow!(impl PowU8, powu8(exp: u8) for Ratio<$t>);
        impl PowI8 for Ratio<$t> {
            #[inline]
            fn powi8(self, exp: i8) -> <Self as Mul>::Output {
                let recip = exp < 0;
                let exp = exp.unsigned_abs();
                let pow = self.powu8(exp);
                if recip { pow.inv() } else { pow }
            }
        }
        forward_ref_pow!(impl PowI8, powi8(exp: i8) for Ratio<$t>);
    };
    ($t: ty) => {
        forward_owned_pow!(impl Square, sq() for Ratio<$t>);
        forward_owned_pow!(impl Cube, cb() for Ratio<$t>);
        forward_owned_pow!(impl PowU8, powu8(exp: u8) for Ratio<$t>);
        impl Square for &Ratio<$t> {
            #[inline]
            fn sq(self) -> Ratio<$t> {
                Ratio::new_raw(self.numer().sq(), self.denom().sq())
            }
        }
        impl Cube for &Ratio<$t> {
            #[inline]
            fn cb(self) -> Ratio<$t> {
                Ratio::new_raw(self.numer().cb(), self.denom().cb())
            }
        }
        impl PowU8 for &Ratio<$t> {
            #[inline]
            fn powu8(self, exp: u8) -> Ratio<$t> {
                Ratio::new_raw(self.numer().powu8(exp), self.denom().powu8(exp))
            }
        }
        forward_owned_pow!(impl PowI8, powi8(exp: i8) for Ratio<$t>);
        impl PowI8 for &Ratio<$t> {
            #[inline]
            fn powi8(self, exp: i8) -> <Self as Mul>::Output {
                let recip = exp < 0;
                let exp = exp.unsigned_abs();
                let pow = self.powu8(exp);
                if recip { pow.inv() } else { pow }
            }
        }
    };
}

impl_pow_for_ratio!(i8: Copy);
impl_pow_for_ratio!(i16: Copy);
impl_pow_for_ratio!(i32: Copy);
impl_pow_for_ratio!(i64: Copy);
impl_pow_for_ratio!(u8: Copy);
impl_pow_for_ratio!(u16: Copy);
impl_pow_for_ratio!(u32: Copy);
impl_pow_for_ratio!(u64: Copy);

impl_pow_for_ratio!(BigInt);
impl_pow_for_ratio!(BigUint);

macro_rules! impl_powu8_for_complex {
    ($t: ty: Copy) => {
        impl Square for Complex<$t> {
            /// (a + bi)² = (a² - b²) + 2ab i
            #[inline]
            fn sq(self) -> Complex<$t> {
                let re = self.re.sq() - self.im.sq();
                let ab = self.re * self.im;
                let im = ab + ab;
                Complex { re, im }
            }
        }
        impl Cube for Complex<$t> {
            /// (a + bi)³ = a(a² - 3b²) + b(3a² - b²) i
            #[inline]
            fn cb(self) -> Complex<$t> {
                let a2 = self.re.sq();
                let b2 = self.im.sq();
                let a2mb2 = a2 - b2;
                let re = self.re * (a2mb2 - (b2 + b2));
                let im = self.im * (a2mb2 + (a2 + a2));
                Complex { re, im }
            }
        }
        impl PowU8 for Complex<$t> {
            #[inline]
            fn powu8(self, exp: u8) -> Complex<$t> {
                static POW: [fn(Complex<$t>) -> Complex<$t>; 256] = pow_array_u8!();
                unsafe { POW.get_unchecked(exp as usize)(self) }
            }
        }
        forward_ref_pow!(impl Square, sq() for Complex<$t>);
        forward_ref_pow!(impl Cube, cb() for Complex<$t>);
        forward_ref_pow!(impl PowU8, powu8(exp: u8) for Complex<$t>);
    };
    ($t: ty) => {
        forward_owned_pow!(impl Square, sq() for Complex<$t>);
        forward_owned_pow!(impl Cube, cb() for Complex<$t>);
        forward_owned_pow!(impl PowU8, powu8(exp: u8) for Complex<$t>);
        impl Square for &Complex<$t> {
            /// (a + bi)² = (a² - b²) + 2ab i
            #[inline]
            fn sq(self) -> Complex<$t> {
                let re = (&self.re).sq() - (&self.im).sq();
                let ab = (&self.re) * (&self.im);
                let im = &ab + &ab;
                Complex { re, im }
            }
        }
        impl Cube for &Complex<$t> {
            /// (a + bi)³ = a(a² - 3b²) + b(3a² - b²) i
            #[inline]
            fn cb(self) -> Complex<$t> {
                let a2 = (&self.re).sq();
                let b2 = (&self.im).sq();
                let a2mb2 = &a2 - &b2;
                let re = (&self.re) * (&a2mb2 - (&b2 + &b2));
                let im = (&self.im) * (&a2mb2 + (&a2 + &a2));
                Complex { re, im }
            }
        }
        impl PowU8 for &Complex<$t> {
            #[inline]
            fn powu8(self, exp: u8) -> Complex<$t> {
                static POW: [fn(&Complex<$t>) -> Complex<$t>; 256] = pow_array_u8!();
                unsafe { POW.get_unchecked(exp as usize)(self) }
            }
        }
    };
}

impl_powu8_for_complex!(BigInt);
impl_powu8_for_complex!(i8);
impl_powu8_for_complex!(i16);
impl_powu8_for_complex!(i32);
impl_powu8_for_complex!(i64);
impl_powu8_for_complex!(f32: Copy);
impl_powu8_for_complex!(f64: Copy);
impl_powu8_for_complex!(Ratio<BigInt>);
impl_powu8_for_complex!(Ratio<i8>: Copy);
impl_powu8_for_complex!(Ratio<i16>: Copy);
impl_powu8_for_complex!(Ratio<i32>: Copy);
impl_powu8_for_complex!(Ratio<i64>: Copy);

impl_powi8_for!(Complex<f32>: Copy);
impl_powi8_for!(Complex<f64>: Copy);
impl_powi8_for!(Complex<Ratio<BigInt>>);
impl_powi8_for!(Complex<Ratio<i8>>);
impl_powi8_for!(Complex<Ratio<i16>>);
impl_powi8_for!(Complex<Ratio<i32>>);
impl_powi8_for!(Complex<Ratio<i64>>);

macro_rules! const_powi_for {
    ($t: ty) => {
        /// A `const` version of `powi` for floats
        pub const fn powi(mut base: $t, exp: i32) -> $t {
            let recip = exp < 0;
            let mut exp = exp.abs_diff(0);
            let mut mul = 1.0;
            loop {
                if (exp & 1) != 0 {
                    mul *= base;
                }
                exp >>= 1;
                if exp == 0 {
                    break;
                }
                base *= base;
            }
            if recip { 1.0 / mul } else { mul }
        }
    };
}

pub mod f32_const {
    //! A `const` version of `powi` for `f32`.
    const_powi_for!(f32);
}
pub mod f64_const {
    //! A `const` version of `powi` for `f64`.
    const_powi_for!(f64);
}

#[cfg(test)]
mod test {
    use num_traits::Pow;

    use super::*;

    macro_rules! assert_close {
        ($this:expr, $that:expr, $d:literal) => {
            assert_eq!(
                format!("{:.p$e}", $this, p = $d),
                format!("{:.p$e}", $that, p = $d)
            )
        };
    }

    #[test]
    fn f64_powu8() {
        for base in [0.5f64, -0.75, 1.25, -2.0] {
            for exp in 0u8..=255 {
                let solution = base.powi(exp as i32);
                assert_close!(solution, base.powu8(exp), 12);
                assert_close!(solution, f64_const::powi(base, exp as i32), 12);
            }
        }
    }

    #[test]
    fn f64_powi8() {
        for base in [0.5f64, -0.75, 1.25, -2.0] {
            for exp in -128i8..=127 {
                let solution = base.powi(exp as i32);
                assert_close!(solution, base.powi8(exp), 12);
                assert_close!(solution, f64_const::powi(base, exp as i32), 13);
            }
        }
    }

    #[test]
    fn big_int_powu8() {
        for base in [2, -3, 5, -8] {
            let base = BigInt::from(base);
            for exp in 0u8..=255 {
                let solution = (&base).pow(exp as u32);
                assert_eq!(solution, (&base).powu8(exp));
            }
        }
    }

    #[test]
    fn ratio_powu8() {
        for base in [(1, 2), (-3, 4), (5, 4), (-2, 1)] {
            let numer = BigInt::from(base.0);
            let denom = BigInt::from(base.1);
            let base = Ratio::new(numer, denom);
            for exp in 0..=255 {
                let solution = (&base).pow(exp as i32);
                assert_eq!(solution, (&base).powu8(exp));
            }
        }
    }

    #[test]
    fn complex_ratio_powu8() {
        for (re, im) in [((1, 2), (-3, 4)), ((5, 4), (-2, 1))] {
            let (numer, denom) = re;
            let re = Ratio::new(BigInt::from(numer), BigInt::from(denom));
            let (numer, denom) = im;
            let im = Ratio::new(BigInt::from(numer), BigInt::from(denom));
            let base = Complex::new(re, im);
            for exp in 0..=255 {
                let result = (&base).powu8(exp);
                let solution = (&base).pow(exp as i32);
                assert_eq!(solution, result);
            }
        }
    }
}
