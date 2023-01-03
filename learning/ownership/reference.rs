fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // transferring the ownership

    println!("The length of '{}' is {}.", s2, len);

    let s3 = String::from("hello");

    let len = calculate_length_again(&s3); // passing the refrence, ownership is stayed


    println!("The length of '{}' is {}.", s3, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_again(s: &String) -> usize {
    s.len()
}
