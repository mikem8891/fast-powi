use num_bigint::{BigInt, BigUint};
use num_complex::Complex;
use num_rational::Ratio;
use num_traits::Inv;
use std::ops::Mul;

#[macro_use]
mod macros;

/// Trait for raising a value to the power of a `u8`.
pub trait PowU8: Mul + Sized {
    /// Raises the value to the power of `exp`.
    fn pow_u8(self, exp: u8) -> <Self as Mul>::Output;
    /// Returns the square of `self`.
    #[inline]
    fn sq(self) -> <Self as Mul>::Output
    where
        Self: Clone,
    {
        self.clone() * self
    }
    /// Returns the cube of `self`.
    #[inline]
    fn cb(self) -> <Self as Mul>::Output
    where
        Self: Clone,
        <Self as Mul>::Output: Mul<Self, Output = <Self as Mul>::Output>,
    {
        self.clone() * self.clone() * self
    }
}

/// Trait for raising a value to the power of an `i8`.
pub trait PowI8: PowU8 {
    #[inline]
    fn pow_i8(self, exp: i8) -> <Self as Mul>::Output
    where
        <Self as Mul>::Output:
            Mul<Self, Output = <Self as Mul>::Output> + Inv<Output = <Self as Mul>::Output>,
    {
        let recip = exp < 0;
        let exp = exp.unsigned_abs();
        let pow = self.pow_u8(exp);
        if recip { pow.inv() } else { pow }
    }
}

macro_rules! forward_ref_pow {
    (impl $imp:ident, $method:ident for $b:ty, $e:ty) => {
        impl $imp for &$b {
            #[inline]
            fn $method(self, exp: $e) -> $b {
                $imp::$method(*self, exp)
            }
        }
    };
}

macro_rules! forward_owned_pow {
    (impl $imp:ident, $method:ident for $b:ty, $e:ty) => {
        impl $imp for $b {
            #[inline]
            fn $method(self, exp: $e) -> $b {
                $imp::$method(&self, exp)
            }
        }
    };
}

macro_rules! impl_powu8_for {
    ($t: ty) => {
        impl PowU8 for $t {
            #[inline]
            fn pow_u8(self, exp: u8) -> $t {
                (&self).pow_u8(exp)
            }
            #[inline]
            fn sq(self) -> $t {
                (&self).sq()
            }
            #[inline]
            fn cb(self) -> $t {
                (&self).cb()
            }
        }
        impl PowU8 for &$t {
            #[inline]
            fn pow_u8(self, exp: u8) -> $t {
                static POW: [fn(&$t) -> $t; 256] = pow_array_u8!();
                unsafe { POW.get_unchecked(exp as usize)(self) }
            }
        }
    };
    ($t: ty: Copy) => {
        impl PowU8 for $t {
            #[inline]
            fn pow_u8(self, exp: u8) -> $t {
                static POW: [fn($t) -> $t; 256] = pow_array_u8!();
                unsafe { POW.get_unchecked(exp as usize)(self) }
            }
        }
        forward_ref_pow!(impl PowU8, pow_u8 for $t, u8);
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
            fn pow_i8(self, exp: i8) -> $t {
                static POW: [fn($t) -> $t; 256] = pow_array_i8!();
                unsafe { POW.get_unchecked(exp as u8 as usize)(self) }
            }
        }
        forward_ref_pow!(impl PowI8, pow_i8 for $t, i8);
    };
}

impl_powi8_for!(f32: Copy);
impl_powi8_for!(f64: Copy);

macro_rules! impl_pow_for_ratio {
    ($t: ty: Copy) => {
        impl PowU8 for Ratio<$t> {
            #[inline]
            fn pow_u8(self, exp: u8) -> Ratio<$t> {
                Ratio::new_raw(self.numer().pow_u8(exp), self.denom().pow_u8(exp))
            }
            #[inline]
            fn sq(self) -> Ratio<$t> {
                Ratio::new_raw(self.numer().sq(), self.denom().sq())
            }
            #[inline]
            fn cb(self) -> Ratio<$t> {
                Ratio::new_raw(self.numer().cb(), self.denom().cb())
            }
        }
        forward_ref_pow!(impl PowU8, pow_u8 for Ratio<$t>, u8);
        impl PowI8 for Ratio<$t> {}
        forward_ref_pow!(impl PowI8, pow_i8 for Ratio<$t>, i8);
    };
    ($t: ty) => {
        forward_owned_pow!(impl PowU8, pow_u8 for Ratio<$t>, u8);
        impl PowU8 for &Ratio<$t> {
            #[inline]
            fn pow_u8(self, exp: u8) -> Ratio<$t> {
                Ratio::new_raw(self.numer().pow_u8(exp), self.denom().pow_u8(exp))
            }
            #[inline]
            fn sq(self) -> Ratio<$t> {
                Ratio::new_raw(self.numer().sq(), self.denom().sq())
            }
            #[inline]
            fn cb(self) -> Ratio<$t> {
                Ratio::new_raw(self.numer().cb(), self.denom().cb())
            }
        }
        forward_owned_pow!(impl PowI8, pow_i8 for Ratio<$t>, i8);
        impl PowI8 for &Ratio<$t> {}
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

macro_rules! impl_pow_for_complex {
    ($t: ty: Copy) => {
        impl PowU8 for Complex<$t> {
            #[inline]
            fn pow_u8(self, exp: u8) -> Complex<$t> {
                static POW: [fn(Complex<$t>) -> Complex<$t>; 256] = pow_array_u8!();
                unsafe { POW.get_unchecked(exp as usize)(self) }
            }
            #[inline]
            fn sq(self) -> Complex<$t> {
                // (a + bi)^2 = (a^2 - b^2) + 2abi
                let re = self.re.sq() - self.im.sq();
                let ab = self.re * self.im;
                let im = ab + ab;
                Complex { re, im }
            }
            #[inline]
            fn cb(self) -> Complex<$t> {
                // (a + bi)^3 = a(a^2 - 3b^2) + b(3a^2 - b^2) i
                let a2 = self.re.sq();
                let b2 = self.im.sq();
                let a2mb2 = a2 - b2;
                let re = self.re * (a2mb2 - (b2 + b2));
                let im = self.im * (a2mb2 + (a2 + a2));
                Complex { re, im }
            }
        }
        forward_ref_pow!(impl PowU8, pow_u8 for Complex<$t>, u8);
        impl_powi8_for!(Complex<$t>: Copy);
    };
    ($t: ty) => {
        impl PowU8 for Complex<$t> {
            #[inline]
            fn pow_u8(self, exp: u8) -> Complex<$t> {
                (&self).pow_u8(exp)
            }
            #[inline]
            fn sq(self) -> Complex<$t> {
                (&self).sq()
            }
            #[inline]
            fn cb(self) -> Complex<$t> {
                (&self).cb()
            }
        }
        impl PowU8 for &Complex<$t> {
            #[inline]
            fn pow_u8(self, exp: u8) -> Complex<$t> {
                static POW: [fn(&Complex<$t>) -> Complex<$t>; 256] = pow_array_u8!();
                unsafe { POW.get_unchecked(exp as usize)(self) }
            }
            #[inline]
            fn sq(self) -> Complex<$t> {
                // (a + bi)^2 = (a^2 - b^2) + 2abi
                let re = (&self.re).sq() - (&self.im).sq();
                let ab = (&self.re) * (&self.im);
                let im = &ab + &ab;
                Complex { re, im }
            }
            #[inline]
            fn cb(self) -> Complex<$t> {
                // (a + bi)^3 = a(a^2 - 3b^2) + b(3a^2 - b^2) i
                let a2 = (&self.re).sq();
                let b2 = (&self.im).sq();
                let a2mb2 = &a2 - &b2;
                let re = (&self.re) * (&a2mb2 - (&b2 + &b2));
                let im = (&self.im) * (&a2mb2 + (&a2 + &a2));
                Complex { re, im }
            }
        }
        forward_owned_pow!(impl PowI8, pow_i8 for Complex<$t>, i8);
        impl PowI8 for &Complex<$t> {
            #[inline]
            fn pow_i8(self, exp: i8) -> Complex<$t> {
                static POW: [fn(&Complex<$t>) -> Complex<$t>; 256] = pow_array_i8!();
                unsafe { POW.get_unchecked(exp as u8 as usize)(self) }
            }
        }
    };
}

impl_pow_for_complex!(f32: Copy);
impl_pow_for_complex!(f64: Copy);
impl_pow_for_complex!(Ratio<BigInt>);
impl_pow_for_complex!(Ratio<i8>: Copy);
impl_pow_for_complex!(Ratio<i16>: Copy);
impl_pow_for_complex!(Ratio<i32>: Copy);
impl_pow_for_complex!(Ratio<i64>: Copy);

macro_rules! const_powi_for {
    ($t: ty) => {
        /// A `const` version of `powi` for floats
        #[inline]
        pub const fn powi(mut a: $t, b: i32) -> $t {
            let recip = b < 0;
            let mut pow = b.abs_diff(0);
            let mut mul = 1.0;
            loop {
                if (pow & 1) != 0 {
                    mul *= a;
                }
                pow >>= 1;
                if pow == 0 {
                    break;
                }
                a *= a;
            }
            if recip { 1.0 / mul } else { mul }
        }
    };
}

pub mod f32_const {
    const_powi_for!(f32);
}
pub mod f64_const {
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
    fn f64_pow_u8() {
        for base in [0.5f64, -0.75, 1.25, -2.0] {
            for exp in 0u8..=255 {
                let solution = base.powi(exp as i32);
                assert_close!(solution, base.pow_u8(exp), 12);
                assert_close!(solution, f64::powi(base, exp as i32), 12);
            }
        }
    }

    #[test]
    fn f64_pow_i8() {
        for base in [0.5f64, -0.75, 1.25, -2.0] {
            for exp in -128i8..=127 {
                let solution = base.powi(exp as i32);
                assert_close!(solution, base.pow_i8(exp), 12);
                assert_close!(solution, f64::powi(base, exp as i32), 12);
            }
        }
    }

    #[test]
    fn big_int_pow_u8() {
        for base in [2, -3, 5, -8] {
            let base = BigInt::from(base);
            for exp in 0u8..=255 {
                let solution = (&base).pow(exp as u32);
                assert_eq!(solution, (&base).pow_u8(exp));
            }
        }
    }

    #[test]
    fn ratio_pow_u8() {
        for base in [(1, 2), (-3, 4), (5, 4), (-2, 1)] {
            let numer = BigInt::from(base.0);
            let denom = BigInt::from(base.1);
            let base = Ratio::new(numer, denom);
            for exp in -128..=127 {
                let solution = (&base).pow(exp as i32);
                assert_eq!(solution, (&base).pow_i8(exp));
            }
        }
    }

    #[test]
    fn complex_ratio_pow_u8() {
        for (re, im) in [((1, 2), (-3, 4)), ((5, 4), (-2, 1))] {
            let (numer, denom) = re;
            let re = Ratio::new(BigInt::from(numer), BigInt::from(denom));
            let (numer, denom) = im;
            let im = Ratio::new(BigInt::from(numer), BigInt::from(denom));
            let base = Complex::new(re, im);
            for exp in -128i8..=127 {
                let result = (&base).pow_i8(exp);
                let solution = (&base).pow(exp as i32);
                assert_eq!(solution, result);
            }
        }
    }
}
