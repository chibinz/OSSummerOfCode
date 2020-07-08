// Use pointers as last resort.
// In circumstances where you must write a lot of code using pointers,
// program in C and then let Rust call C functions might be a better idea.

fn main() {
    let ages = [23, 43, 12, 89, 2];
    let names = ["Alan", "Frank", "Mary", "John", "Lisa"];

    let mut cur_age = ages.as_ptr();
    let mut cur_name = names.as_ptr();

    // Pointer derefencing need to be done in unsafe block
    unsafe {
        for _ in 0..5 {
            println!("{} {}", *cur_age, *cur_name);

            // Pointer addition is actually adding the element size
            // to the pointer
            cur_age = cur_age.add(1);
            cur_name = cur_name.add(1);
        }
    }
}