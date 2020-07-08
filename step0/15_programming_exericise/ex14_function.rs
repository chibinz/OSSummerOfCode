use std::env::args;

fn print_arguments() {
    args().for_each(|arg| println!("{}", arg));
}

// Mutually recursive function
fn is_odd(n: u32) -> bool {
    if n == 0 {
        return false;
    } else {
        return is_even(n - 1);
    }
}

fn is_even(n: u32) -> bool {
    if n == 0 {
        return true;
    } else {
        return is_odd(n - 1);
    }
}

fn main() {
    print_arguments();

    println!("{}", is_odd(34));
}
