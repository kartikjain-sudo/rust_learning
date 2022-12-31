fn tuple() {
    // let x:(u32, str, f32) = (50, "kartik", 7.6); // throws error for string (doesn't have a size known at compile-time)
    let x = (50, "kartik", 7.6);
    println!("{} {} {}", x.0, x.1, x.2);

    let (a, b, c) = x;
    println!("{} {} {}", c, a, b);
}

fn array() {
    let a:[i32; 5] = [1,2,3,4,5]; 
    let b = [3; 5];

    println!("{} {} {}", a[3], b[0], b[4])
}

fn main() {
    tuple();
    array();
}