use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let argc = args.len();

    if argc != 2 {
        println!("2 arguments");
        return;
    }

    // Capitalize and print all vowels
    for i in 0..args[1].len() {
        // Rust doesn't allow you to index a String because chars
        // might not always be a byte depending on the encoding
        match args[1].chars().nth(i).unwrap() {
            'a' | 'A' => println!("{}: A", i),
            'e' | 'E' => println!("{}: E", i),
            'i' | 'I' => println!("{}: I", i),
            'o' | 'O' => println!("{}: O", i),
            'u' | 'U' => println!("{}: U", i),
            _         => (), // Rust match pattern must be exhaustive
        }
    }
}