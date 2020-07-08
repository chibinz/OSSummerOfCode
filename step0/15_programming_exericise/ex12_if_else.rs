use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc == 1 {
        println!("You only have one argument. You suck.");
    } else if argc > 1 && argc < 4 {
        println!("Here's your arguments: ");
        args.iter().enumerate().map(|(i, arg)| println!("arg {}: {}", i, arg)).for_each(drop);
    } else {
        println!("You have too many arguments. You suck.");
    }
}