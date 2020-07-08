/// Rust allows you to write function pointer(type) in an elegant and simple way
type CmpFn = fn(i32, i32) -> bool;

fn bubble_sort(numbers: &mut [i32], cmp: CmpFn) {
    let len = numbers.len();

    for _ in 0..len {
        for j in 0..(len - 1) {
            if cmp(numbers[j], numbers[j+1]) {
                let temp = numbers[j+1];
                numbers[j+1] = numbers[j];
                numbers[j] = temp;
            }
        }
    }
}

fn inc_cmp(a: i32, b: i32) -> bool {
    a > b
}

fn dec_cmp(a: i32, b: i32) -> bool {
    a < b
}

fn main() {
    let mut arr = [3, 2, 8, 5, 0, 9];
    dbg!(arr);

    bubble_sort(&mut arr, inc_cmp);
    dbg!(arr);

    bubble_sort(&mut arr, dec_cmp);
    dbg!(arr);
}