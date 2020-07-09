
fn main() {
    let a: u32 = 2;
    let ptr2a= &a as *const u32;

    dbg!(ptr2a);

    let b = Box::new(2);
    let ptr2b = Box::into_raw(b);

    dbg!(ptr2b);

    let v = vec![1; 10];
    dbg!(v.as_ptr());
}