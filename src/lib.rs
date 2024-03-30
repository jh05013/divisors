//! # divisors_fixed
//!
//! This is a clone of the divisors crate with the following differences:
//!
//! - The bug has been fixed for n = 2
//! - The function was changed to use the proper definition of divisors, so it includes 1 and n
//!
//! [The original crate](https://github.com/uccidibuti/divisors) says:
//!
//! > A blazing fast Rust library to find all divisors of a natural number. This library works with u8, u16, u32, u64, u128 and usize types.

pub trait Divisors: Sized {
    fn divisors_unordered(&self) -> Vec<Self>;

    fn divisors(&self) -> Vec<Self> where Self: Ord {
        let mut v = self.divisors_unordered();
        v.sort();
        v
    }
}

macro_rules! impl_divisors {
    ($t:ty) => {
        impl Divisors for $t {
            fn divisors_unordered(&self) -> Vec<Self> {
                fn repeat_division(mut n: $t, d: $t) -> ($t, usize) {
                    for rep in 0.. {
                        // this will be optimized by LLVM
                        let (new_n, rem) = (n / d, n % d);

                        if rem != 0 {
                            return (n, rep);
                        }
                        n = new_n;
                    }
                    unreachable!();
                }

                fn multiply_and_extend(v: &mut Vec<$t>, multiplier: $t, rep: usize) {
                    v.reserve(v.len() * (rep + 1));
                    for i in 0..v.len() * rep {
                        let vi = unsafe { v.get_unchecked(i) };
                        let new_div = vi * multiplier;
                        v.push(new_div);
                    }
                }

                if *self == 0 {
                    return vec![];
                }

                let mut v = vec![1];

                let mut n = *self;
                for d in std::iter::once(2).chain((3..).step_by(2)) {
                    if d * d > n {
                        break;
                    }
                    let (new_n, rep) = repeat_division(n, d);
                    n = new_n;
                    multiply_and_extend(&mut v, d, rep);
                }
                if n > 1 {
                    multiply_and_extend(&mut v, n, 1);
                }

                v
            }
        }
    };
}

impl_divisors!(u8);
impl_divisors!(u16);
impl_divisors!(u32);
impl_divisors!(u64);
impl_divisors!(u128);
impl_divisors!(usize);
