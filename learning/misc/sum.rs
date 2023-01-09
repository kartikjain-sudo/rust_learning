fn sum(vector: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for item in vector {
        sum = sum + item
    }
    sum
}

fn main() {
    let v = vec![1,2,3];
    let s = sum(&v);

    println!("sum of {:?}: {}", v, s); 
}