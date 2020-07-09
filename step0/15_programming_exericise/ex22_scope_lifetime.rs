// Rust's lifetime rules are far more complicated and drastically from C's
// So the following code tries out examples listed in Rust By Example
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // 'c' is destroyed and the memory freed
}

fn borrow(borrowed: &i32) {
    println!("This int is: {}", borrowed);
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let x = 5;
    let y = x;

    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5);

    println!("a contains: {}", a);

    let b = a;

    destroy_box(b);
    // now b and a are no longer accessible

    let immutable_box = Box::new(5);

    println!("immutable_box contains {}", immutable_box);

    let mut mutable_box = immutable_box;

    // asterick * essential
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);

    let boxed = Box::new(5);
    let stacked = 6;

    borrow(&boxed);
    borrow(&stacked);

    let immutabook = Book {
        author: "Douglas",
        title: "Godel",
        year: 1979,
    };

    let mut mutabook = immutabook;

    new_edition(&mut mutabook);

    println!("{}", mutabook.year);

    let c = 'Q';

    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    let _copy_of_x = {
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;

        *ref_to_x
    };

    let mut mutable_point = point;

    {
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;

        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );
}
