// Adapted from Rust By Example

use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("xs: {:?} ys: {:?}", xs, &ys[..]);

    analyze_slice(&xs);
    analyze_slice(&ys[1..5]);
}
