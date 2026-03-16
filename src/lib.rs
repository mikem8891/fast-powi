use num_bigint::{BigInt, BigUint};
use num_traits::Inv;

#[macro_use]
mod macros;

pub trait PowU8 {
    type Output;
    fn pow_u8(self, exp: u8) -> Self::Output;
}

pub trait PowI8: PowU8 {
    fn pow_i8(self, exp: i8) -> Self::Output;
}

macro_rules! impl_powu8_for {
    ($t: ty) => {
        impl PowU8 for $t {
            type Output = $t;
            #[inline]
            fn pow_u8(self, exp: u8) -> $t {
                (&self).pow_u8(exp)
            }
        }
        impl PowU8 for &$t {
            type Output = $t;
            #[inline]
            fn pow_u8(self, exp: u8) -> $t {
                static POW: [fn(&$t) -> $t; 256] = pow_array_u8!();
                unsafe { POW.get_unchecked(exp as usize)(self) }
            }
        }
    };
    ($t: ty: Copy) => {
        impl PowU8 for $t {
            type Output = $t;
            #[inline]
            fn pow_u8(self, exp: u8) -> $t {
                static POW: [fn($t) -> $t; 256] = pow_array_u8!();
                unsafe { POW.get_unchecked(exp as usize)(self) }
            }
        }
        impl PowU8 for &$t {
            type Output = $t;
            #[inline]
            fn pow_u8(self, exp: u8) -> $t {
                (*self).pow_u8(exp)
            }
        }
    };
}

macro_rules! impl_powi8_for {
    ($t: ty) => {
        impl PowI8 for $t {
            #[inline]
            fn pow_i8(self, exp: i8) -> $t {
                (&self).pow_i8(exp)
            }
        }
        impl PowI8 for &$t {
            #[inline]
            fn pow_i8(self, exp: i8) -> $t {
                static POW: [fn(&$t) -> $t; 256] = pow_array_i8!();
                unsafe { POW.get_unchecked(exp as usize)(self) }
            }
        }
    };
    ($t: ty: Copy) => {
        impl PowI8 for $t {
            #[inline]
            fn pow_i8(self, exp: i8) -> $t {
                static POW: [fn($t) -> $t; 256] = pow_array_i8!();
                unsafe { POW.get_unchecked(exp as usize)(self) }
            }
        }
        impl PowI8 for &$t {
            #[inline]
            fn pow_i8(self, exp: i8) -> $t {
                (*self).pow_i8(exp)
            }
        }
    };
}

impl_powu8_for!(f32: Copy);
impl_powi8_for!(f32: Copy);
impl_powu8_for!(f64: Copy);
impl_powi8_for!(f64: Copy);
impl_powu8_for!(i8: Copy);
impl_powu8_for!(i16: Copy);
impl_powu8_for!(i32: Copy);
impl_powu8_for!(i64: Copy);
impl_powu8_for!(BigInt);
impl_powu8_for!(u8: Copy);
impl_powu8_for!(u16: Copy);
impl_powu8_for!(u32: Copy);
impl_powu8_for!(u64: Copy);
impl_powu8_for!(BigUint);

macro_rules! const_powi_for {
    ($t: ty) => {
        pub const fn powi(a: $t, b: i32) -> $t {
            let mut a = a;
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

pub mod f32 {const_powi_for!(f32);}
pub mod f64 {const_powi_for!(f64);}

#[cfg(test)]
mod test {
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
    fn pow_u8() {
        for base in [0.5f64, 0.9, 1.1, 1.5] {
            for exp in 0u8..=255 {
                let solution = base.powi(exp as i32);
                assert_close!(solution, base.pow_u8(exp), 11);
                assert_close!(solution, f64::powi(base, exp as i32), 11);
            }
        }
    }

    #[test]
    fn pow_i8() {
        for base in [0.5f64, 0.9, 1.1, 1.5] {
            for exp in -128i8..=127 {
                let solution = base.powi(exp as i32);
                assert_close!(solution, base.pow_i8(exp), 11);
                assert_close!(solution, f64::powi(base, exp as i32), 11);
            }
        }
    }
}
