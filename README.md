# fizzbuzz

The least complicated, most feature complete fizzbuzz ever written.

## Usage

Add `fizzbuzz = { git = "https://github.com/mikopits/fizzbuzz" }` to your `Cargo.toml`.

## Example

```rust
extern crate fizzbuzz;

fn main() {
    let mut fb = ::fizzbuzz::FizzBuzz::new(1, 100);
    fb.print();
}
```
