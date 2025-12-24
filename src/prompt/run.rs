use std::io::{Write, stdin, stdout};

pub fn run() {
    print!("Enter your name: ");
    stdout().flush().unwrap();

    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();

    let name = name.trim();

    println!("Hello, {name}!");
}
