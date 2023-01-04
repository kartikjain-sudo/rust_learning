use std::io;

fn fib(num: u32) {

    let mut a = 0;
    let mut b = 1;

    let mut i = 0;
    let mut c = 0;

    if num == 0 { return; }

    if num == 1 {
        println!("{c} ");
        return;
    }
    else if num == 2 {
        println!("{a} {b} ");
        return;
    }

    print!("{a} {b} ");

    while i < num-2 {
        c = a + b;
        a = b;
        b = c;
        print!("{} ", c.to_string().trim());
        i += 1;
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
        Err(..) => 0,
    };

    fib(num);
}
