fn tuple() {
    // let x:(u32, str, f32) = (50, "kartik", 7.6); // throws error for string (doesn't have a size known at compile-time)
    let x = (50, "kartik", 7.6);
    println!("{} {} {}", x.0, x.1, x.2);

    let (a, b, c) = x;
    println!("{} {} {}", c, a, b);
}

fn main() {
    tuple();
}