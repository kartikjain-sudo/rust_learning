pub fn shadowing() {
    // defining the first variable as integer
    let first = 50;
    println!("I am {}", first);

    // defining again as string
    // this is shadowing
    let first = "AMAR";
    println!("I am {}", first);
}

fn main() {
    shadowing()
}