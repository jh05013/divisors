//! # divisors_fixed
//!
//! A minimalistic crate for finding all divisors of an integer.
//! Although its worst-case time complexity is (currently)
//! still O(sqrt(n)), It is practically much faster than the
//! naive method of trying every integers under sqrt(n).
//!
//! This library works with `u8`, `u16`, `u32`, `u64`, `u128`,
//! and `usize` types.
//!
//! This originally started as a clone of the
//! [divisors](https://github.com/uccidibuti/divisors) crate with
//! bug fixes (hence the name), but has since evolved with
//! API changes and reimplementations.
//!
//! # Example
//! ```
//! use divisors_fixed::Divisors;
//! assert_eq!(12u32.divisors(), vec![1, 2, 3, 4, 6, 12]);
//! ```

/// Trait for getting all positive divisors of an integer.
pub trait Divisors: Sized {
    /// Returns the list of divisors of `self` in no particular order.
    fn divisors_unordered(&self) -> Vec<Self>;

    /// Returns the list of divisors of `self` in the ascending order.
    fn divisors(&self) -> Vec<Self>
    where
        Self: Ord,
    {
        let mut v = self.divisors_unordered();
        v.sort_unstable();
        v
    }
}

macro_rules! impl_divisors {
    ($t:ty) => {
        impl Divisors for $t {
            fn divisors_unordered(&self) -> Vec<Self> {
                #[inline]
                /// Keeps dividing `n` by `d` until you can't.
                /// Returns the final value of `n` and the amount divided.
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

                #[inline]
                /// Expands the vector `v` by `rep` times by multiplying
                /// each element by `multiplier`.
                fn multiply_and_extend(v: &mut Vec<$t>, multiplier: $t, rep: usize) {
                    v.reserve(v.len() * (rep + 1));
                    for i in 0..v.len() * rep {
                        // SAFETY: i < v.len() is always true
                        let vi = unsafe { v.get_unchecked(i) };
                        v.push(vi * multiplier);
                    }
                }

                #[inline]
                /// Returns the possible prime number candidates.
                fn candidates() -> impl Iterator<Item = $t> {
                    [2, 3, 5].into_iter().chain(
                        (7..).step_by(30).flat_map(|d| {
                            [d, d + 4, d + 6, d + 10, d + 12, d + 16, d + 22, d + 24]
                        }),
                    )
                }

                if *self == 0 {
                    return vec![];
                }

                let mut divisors = vec![1];

                let mut n = *self;
                for d in candidates() {
                    if !d.checked_mul(d).is_some_and(|res| res <= n) {
                        break;
                    }
                    let (new_n, rep) = repeat_division(n, d);
                    n = new_n;
                    multiply_and_extend(&mut divisors, d, rep);
                }
                if n > 1 {
                    multiply_and_extend(&mut divisors, n, 1);
                }

                divisors
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

#[cfg(test)]
mod test {
    use crate::Divisors;

    fn do_test(n: u32) {
        assert_eq!(n.divisors(), get_divisors_standard(n));
    }

    #[test]
    fn test_smallest() {
        for i in 0..10 {
            do_test(i);
        }
    }

    #[test]
    fn test_small() {
        for i in 10..100000 {
            do_test(i);
        }
    }

    #[test]
    fn test_near_max() {
        for i in u32::MAX - 1000..=u32::MAX {
            do_test(i);
        }
    }

    /// geeksforgeeks solution: https://www.geeksforgeeks.org/find-divisors-natural-number-set-1/
    fn get_divisors_standard(n: u32) -> Vec<u32> {
        let mut v = Vec::new();
        let n_sqrt = (n as f32).sqrt() as u32 + 1;

        for i in 1..n_sqrt {
            if n % i == 0 {
                if n / i == i {
                    v.push(i);
                } else {
                    v.push(i);
                    v.push(n / i);
                }
            }
        }
        v.sort();
        v
    }
}
