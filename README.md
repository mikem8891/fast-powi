The `fast-powi` crate provides fast exponentiation for numeric types.

This crate provides a `const` version of `powi` for floats, as well as optimized implementations for `u8` and `i8` exponents for numeric types. Exponentiation is optimized using a table of shortest multiplication chains. `Square` and `Cube` traits are also provided for further optimization of types that can be squared and cubed more efficiently than by repeated multiplication, such as complex numbers.
