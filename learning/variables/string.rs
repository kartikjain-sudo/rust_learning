use std::io;

fn read_string() {
    let mut input = String::new();
    println!("Enter your name");
    
    io::stdin().read_line(&mut input).expect("string is expected");
    
    // input = input.trim().to_string();
    // index.push_str(", ok");
    
    println!("{}", input);
}

fn string_mut() {
    let mut text = String::from("hello");

    text.push_str(", world!");
    println!("{}", text);
}

fn main() {
    // read_string();
    string_mut();
}