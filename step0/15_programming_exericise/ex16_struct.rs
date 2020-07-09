#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    height: u32,
    weight: u32,
}

impl Person {
    // Rust does not have a special constructor function
    // like C++. The convention is to write a pseoudo constructor
    // function named new.
    fn new(n: &str, a: u32, h: u32, w: u32) -> Self {
        Self {
            name: String::from(n),
            age: a,
            height: h,
            weight: w,
        }
    }

    // Likewise, Rust also doesn't have a destructor, because memory
    // allocated when be automatically freed when a variable's lifetime
    // expires.Person
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "name: {}, age: {}, height: {}, weight: {}",
                   self.name, self.age, self.height, self.weight)
    }
}

fn main() {
    let joe = Person::new("Joe", 32, 64, 140);
    let frank = Person {
        name: String::from("Frank Blank"),
        age: 20,
        height: 72,
        weight: 180,
    };

    println!("Joe: {}", joe);

    dbg!(&frank);
}