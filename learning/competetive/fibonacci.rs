use std::io;

fn fib(num: u32) {

    let mut a = 0;
    let mut b = 1;

    let mut i = 0;
    let mut c = 0;

    if i == 0 { return; }

    if i == 1 {
        println!("{c} ");
        return;
    }
    else if i == 2 {
        println!("{a} {b} ");
        return;
    }

    println!("{a} {b} ");

    while i < num-2 {
        c = a + b;
        a = b;
        b = c;
        println!("{c} ");
    }
    return;
}

fn main() {
    let mut num = String::new();

    io::stdin().
        read_line(&mut num).
        expect("Not a num");

    

    let num = match num.trim().parse::<u32>() {
        Ok(i) => i,
        Err(..) => println!("Not a num"),
    };

    // fib(num);

    println!("{}", num);
}
