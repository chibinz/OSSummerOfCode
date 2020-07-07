// include <stdio.h>

// int main(int argc, char *argv[])
// {
//     int distance = 100;
//     float power = 2.345f;
//     double super_power = 56789.4532;
//     char initial = 'A';
//     char first_name[] = "Zed";
//     char last_name[] = "Shaw";

//     printf("You are %d miles away.\n", distance);
//     printf("You have %f levels of power.\n", power);
//     printf("You have %f awesome super powers.\n", super_power);
//     printf("I have an initial %c.\n", initial);
//     printf("I have a first name %s.\n", first_name);
//     printf("I have a last name %s.\n", last_name);
//     printf("My whole name is %s %c. %s.\n",
//             first_name, initial, last_name);

//     return 0;
// }

/// Rust does type inference automatically for local variables within
/// a function. However you can always annotate a let binding with a
/// postfix explicit type declaration
fn main() {
    let dist: i32 = 100;
    let power: f32 = 2.345;
    // A double precision floating point number is 64 bits
    // according to IEEE 754 specification
    let super_power: f64 = 56789.4532;
    // A char is encoded in utf-8 in rust, thus changing the type annotation
    // from char to i8 will not compile
    let initial: char = 'A';
    // Static string
    let first_name: &str = "Zed";
    let last_name: &str = "Shaw";

    println!("{} {} {} {} {} {}", dist, power, super_power,
            initial, first_name, last_name);
}