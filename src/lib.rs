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

pub trait Num: NumCast + Unsigned + Copy + PrimInt + Display {}

impl Num for u8 {}
impl Num for u16 {}
impl Num for u32 {}
impl Num for u64 {}
impl Num for u128 {}
impl Num for usize {}

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
pub fn get_divisors<T: Num>(n: T) -> Vec<T> {
    if n == T::zero() {
        return vec![];
    }

    let zero: T = T::zero();
    let one: T = T::one();
    let two: T = T::from(2).unwrap();
    let mut _n = n;
    let mut v: Vec<T> = Vec::new();

    let mut count_divisors_2: usize = 0;
    while _n & one == zero {
        v.push(two << count_divisors_2);
        count_divisors_2 += 1;
        _n = _n >> 1;
    }

    let mut _x: T = T::from(3).unwrap();
    let mut _n_sqrt: T = approximated_sqrt(_n);
    while _x < _n_sqrt {
        let mut _pow_x = _x;
        let v_len = v.len();
        let mut x_is_a_divisors = false;

        let mut pow_x_is_a_divisors = _n % _x == zero;
        while pow_x_is_a_divisors {
            _n = _n.div(_x);
            v.push(_pow_x);
            push_new_divisors(&mut v, v_len, _pow_x);
            pow_x_is_a_divisors = _n % _x == zero;
            if pow_x_is_a_divisors {
                _pow_x = _pow_x.mul(_x);
            }
            x_is_a_divisors = true;
        }
        _x = _x + two;
        if x_is_a_divisors {
            _n_sqrt = approximated_sqrt(_n);
        }
    }

    if _n > one && _n != n {
        let v_len = v.len();
        v.push(_n);
        push_new_divisors(&mut v, v_len, _n);
    }

    if v.is_empty() {
        v.push(n);
    }
    if n != one {
        v.push(one);
    }
    v.sort();

    v
}

fn approximated_sqrt<T: Num>(n: T) -> T {
    let zero: T = T::zero();
    let one: T = T::one();
    let mut num_bits = (std::mem::size_of::<T>() << 3) - 1;
    while ((n >> num_bits) & one) == zero {
        num_bits -= 1;
    }

    one << ((num_bits >> 1) + 1)
}

/// Iterate on v from 0 to v_len and for each element e: push (e * _x) in v.
fn push_new_divisors<T: Num>(v: &mut Vec<T>, v_len: usize, _x: T) {
    for i in 0..v_len {
        v.push(_x.mul(unsafe { *v.get_unchecked(i) }));
    }
}
