use std::io;

fn tuple() {
    // let x:(u32, str, f32) = (50, "kartik", 7.6); // throws error for string (doesn't have a size known at compile-time)
    let x = (50, "kartik", 7.6);
    println!("{} {} {}", x.0, x.1, x.2);

    let (a, b, c) = x;
    println!("{} {} {}", c, a, b);
}

fn array(index: usize) {
    let a:[i32; 5] = [1,2,3,4,5]; 
    
    let b = [3; 5];  // => [3, 3, 3, 3, 3]

    println!("{} {} {}", b[1], a[index], b[4])
}

fn main() {
    tuple();

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed");

    let index: usize = index
        .trim()
        .parse()
        .expect("not a num");

    array(index);
}