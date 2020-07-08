use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Note this is actually bad style in Rust.
    // You should really use iterators instead of induction variables.
    // Not only is it faster, but also eliminates the need for array bounds check.
    // For this simple snippet, the compiler will be smart enough to optimize out
    // the index.
    // If you run clippy, it will almost definitely throw a warning
    let mut i = 0;
    while i < args.len() {
        println!("arg {}: {}", i, args[i]);
        i += 1;
    }
}