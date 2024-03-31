# Divisors-fixed
A minimalistic crate for finding all divisors of an integer.
Although its worst-case time complexity is (currently)
still O(sqrt(n)), It is practically much faster than the
naive method of trying every integers under sqrt(n).

This library works with `u8`, `u16`, `u32`, `u64`, `u128`,
and `usize` types.

This originally started as a clone of the
[divisors](https://github.com/uccidibuti/divisors) crate with
bug fixes (hence the name), but has since evolved with
API changes and reimplementations.

## Example
``` Rust
let n = 240u32;
println!("{:?}", n.divisors_unordered());
println!("{:?}", n.divisors());

/*
Output:
[1, 2, 4, 8, 16, 3, 6, 12, 24, 48, 5, 10, 20, 40, 80, 15, 30, 60, 120, 240]
[1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 16, 20, 24, 30, 40, 48, 60, 80, 120, 240]
*/
```
## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
divisors = "0.4.0"
```

## Documentations
```sh
cargo doc --open
```
[link](https://docs.rs/divisors_fixed/) 

## Benchmarks
```sh
cargo bench
```

## Test
```sh
cargo test
```

## License
MIT.
