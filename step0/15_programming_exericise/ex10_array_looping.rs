use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for (i, arg) in args.iter().enumerate() {
        println!("arg {}: {}", i, arg);
    }

    let states = ["California", "Oregon", "Washington", "Texas"];

    // Iterating over a list in a functional way
    states
        .iter()
        .enumerate()
        .map(|(i, state)| println!("state {}: {}", i, state))
        .for_each(drop); // The result is of little concern, just consume the iterator returned by map
}
