//! math related helpers

macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

macro_rules! divrems_for {
    ($ty:ident ) => {
        doc_comment! {
            concat!("Math helpers for integer type ",stringify!($ty)),
            pub mod $ty {
                doc_comment! {
                    concat!("Calculates the quotient and remainder of regular integer division and returns the result as a tuple.

Internally it does simply `(lhs / rhs, lhs % rhs)`.

## Panics

This function will panic if `rhs` is 0.

## Example
```rust
let (q, r) = unstd::math::",stringify!($ty),"::divrem(&3, &2);
//=> (1, 1)
```
                    "),
                    pub fn divrem(lhs: &$ty, rhs: &$ty) -> ($ty, $ty) {
                        (lhs / rhs, lhs % rhs)
                    }
                }

                doc_comment! {
                    concat!("Calculates the quotient and remainder of Euclidean division and returns the result as a tuple.

Internally it calls both [`div_euclid`][div_euclid] and [`rem_euclid`][rem_euclid], so same rules apply as described in the rust-lang docs for the type.

## Panics

This function will panic if `rhs` is 0.

## Example
```rust
let (q, r) = unstd::math::",stringify!($ty),"::divrem(&3, &2);
//=> (1, 1)
```

[div_euclid]: ",stringify!($ty),"::div_euclid
[rem_euclid]: ",stringify!($ty),"::rem_euclid
                    "),
                    pub fn divrem_euclid(lhs: &$ty, rhs: &$ty) -> ($ty, $ty) {
                        (lhs.div_euclid(*rhs), lhs.rem_euclid(*rhs))
                    }
                }
            }
        }
    };
}

macro_rules! create_fns_for_ints {
  ($( $ty:ident ),*) => {
    $(
      divrems_for!($ty);
    )*
  };
}

create_fns_for_ints!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
