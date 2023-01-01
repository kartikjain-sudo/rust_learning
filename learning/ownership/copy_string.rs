fn copy_num() {

    let x = 5;
    let y = x; // the data is copied as it has fixed size

    println!("{} {}", x, y);
}

fn copy_string() {

    let x = String::from("hello");

    println!("{}", x);

    let y = x; // here ownership is transferred to y (MOVE)

    // println!("{}", x); -- throws error as 'y' has BORROWED 'x' value

    let z = y.clone(); // this is expensive

    println!("{} {}", y, z); // this works fine 
}

fn main() {
    // copy_num();
    copy_string();
}