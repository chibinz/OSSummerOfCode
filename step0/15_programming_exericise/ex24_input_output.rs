#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    height: u32,
}

impl Person {
    #[allow(dead_code)]
    fn new(n: &str, a: u32, h: u32) -> Self {
        Self {
            name: String::from(n),
            age: a,
            height: h,
        }
    }
}

fn main() {
    let mut you = Person {name: String::from("C"), age: 0, height: 1};
    let mut tmp = String::new();

    println!("What's your name?");
    std::io::stdin().read_line(&mut you.name).unwrap();

    println!("What's your age?");
    std::io::stdin().read_line(&mut tmp).unwrap();
    // Use trim_end to get rid of carriage return
    you.age = tmp.trim_end().parse().unwrap(); // Type inference does the job automatically

    // read_line actually appends to the end of the string
    // instead of overwriting it.
    tmp.clear();
    println!("What's your height?");
    std::io::stdin().read_line(&mut tmp).unwrap();
    you.height = tmp.trim_end().parse().unwrap();

    dbg!(&you);
}