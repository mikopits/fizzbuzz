extern crate fizzbuzz;

fn main() {
    // Conformist fizzbuzz with the standard [1, 100] range.
    let mut fb = fizzbuzz::FizzBuzz::new(1, 100);

    // Print fizzbuzz in [1, 100].
    fb.print();

    // So, when do I start?
}
