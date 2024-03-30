extern crate divisors_fixed;
use divisors_fixed::Divisors;

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
fn test_large() {
    for i in u32::MAX - 1..=u32::MAX {
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
