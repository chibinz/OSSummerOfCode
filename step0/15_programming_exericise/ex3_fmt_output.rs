/// Learn C the hard way excerise 3 reimplemented in Rust
///
/// ```
/// #include <stdio.h>
/// int main()
/// {
///     int age = 10;
///     int height = 72;
///
///     printf("I am %d years old.\n", age);
///     printf("I am %d inches tall.\n", height);
///
///     return 0;
/// }
/// ```

fn main() {

    let age = 20;
    let height = 180;

    println!("I am {} years old.", age);
    println!("I am {} centimeters tall.", height);
}

/// ```
/// Output
/// I am 20 years old.
/// I am 180 centimeters tall.
/// ```