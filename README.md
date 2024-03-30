# Divisors-fixed
This is a clone of the `divisors` crate with the following differences:
- The bug has been fixed for `n = 2`
- The function was changed to use the proper definition of divisors, so it includes 1 and `n`

[The original crate](https://github.com/uccidibuti/divisors) says:

> A blazing fast Rust library to find all divisors of a natural number. This library works with u8, u16, u32, u64, u128 and usize types.

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
divisors = "0.3.0"
```

## Documentations
```sh
cargo doc --no-deps --open
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

## Example
```sh
cargo run --release --example example
```
## License
MIT.
