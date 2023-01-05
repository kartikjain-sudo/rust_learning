use std::io;

fn fib(num: u32) {

    let mut a = 0;
    let mut b = 1;

    match num {
        0 => {
            return;
        },
        1 => {
            println!("0 ");
            return;
        },
        2 => {
            println!("0 1 ");
            return;
        },
        _ => (),
    }

    print!("{a} {b} ");

    for _ in 2..num {
        let c = a + b;
        a = b;
        b = c;
        print!("{} ", c.to_string().trim());
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
