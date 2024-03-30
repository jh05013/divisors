use divisors_fixed::Divisors;

fn main() {
    let n = 240u32;
    println!("{:?}", n.divisors_unordered());
    println!("{:?}", n.divisors());
}

/*
Output:
[1, 2, 4, 8, 16, 3, 6, 12, 24, 48, 5, 10, 20, 40, 80, 15, 30, 60, 120, 240]
[1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 16, 20, 24, 30, 40, 48, 60, 80, 120, 240]
*/
