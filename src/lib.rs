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

use num_traits::{NumCast, PrimInt, Unsigned};
use std::fmt::Display;
use std::iter;

pub trait Divisors: Sized {
    fn divisors(&self) -> Vec<Self>;
}

impl Divisors for u32 {
    fn divisors(&self) -> Vec<Self> {
        fn repeat_division(mut n: u32, d: u32) -> (u32, usize) {
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

            v.reserve(v.len() * (rep + 1));
            for i in 0..v.len() * rep {
                let vi = unsafe { v.get_unchecked(i) };
                let new_div = vi * d;
                v.push(new_div);
            }
        }
        if n > 1 {
            v.reserve(v.len() * 2);
            for i in 0..v.len() {
                let vi = unsafe { v.get_unchecked(i) };
                let new_div = vi * n;
                v.push(new_div);
            }
        }

        v.sort_unstable();
        v
    }
}

impl Divisors for u64 {
    fn divisors(&self) -> Vec<Self> {
        fn repeat_division(mut n: u64, d: u64) -> (u64, usize) {
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

            v.reserve(v.len() * (rep + 1));
            for i in 0..v.len() * rep {
                let vi = unsafe { v.get_unchecked(i) };
                let new_div = vi * d;
                v.push(new_div);
            }
        }
        if n > 1 {
            v.reserve(v.len() * 2);
            for i in 0..v.len() {
                let vi = unsafe { v.get_unchecked(i) };
                let new_div = vi * n;
                v.push(new_div);
            }
        }

        v.sort_unstable();
        v
    }
}

/// Return a vector with all divisors ordered of n in range [1, n].
///
/// # Example
///
/// ```
/// use std::time::{Instant};
///
/// let n: u128 = 934832147123321;
/// println!("finding divisors of {}", n);
/// let start_time = Instant::now();
/// let v = divisors_fixed::get_divisors(n);
/// println!("time = {:?}, divisors = {:?}", start_time.elapsed(), v);
/// ```
pub fn get_divisors(n: u32) -> Vec<u32> {
    n.divisors()
}
