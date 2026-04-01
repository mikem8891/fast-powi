The `fast-powi` crate provides fast exponentiation for numeric types.

This crate provides a `const` version of `powi` for floats, as well as optimized implementations for `u8` and `i8` exponents for numeric types. Exponentiation is optimized using a table of shortest multiplication chains. `Square` and `Cube` traits are also provided for further optimization of types that can be squared and cubed more efficiently than by repeated multiplication, such as complex numbers.

# Example
```rust
use fast_powi::PowI8;
use num_complex::Complex;
use num_rational::Ratio;
use num_traits::ConstOne;

const ONE: Complex<Ratio<i32>> = ConstOne::ONE;
let a = Complex::new(Ratio::new(3, 5), Ratio::new(4, 5));
assert_eq!(a.powi8(-5), ONE / (a * a * a * a * a));
```

# Features

This crate does not use the standard library; i.e., it is `no_std`.  By default `fast-powi` supports numeric primatives.  Through features, `fast-powi` supports `num` crate numeric types. The features for the individual `num` crate numeric types are `big-int`, `complex`, and `ratio`. The feature `num` is for all the `num` crate numeric types.
