use std::io;

fn read_string() {
    let mut input = String::new();
    println!("Enter your name");
    io::stdin().read_line(&mut input).expect("string is expected");

    println!("{}", input);
}

fn main() {
    read_string();
}