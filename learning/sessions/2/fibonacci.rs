use std::io;

fn fib(num: u32) -> Vec<i32> {

    let mut res = Vec::new();
    let mut a = 0;
    let mut b = 1;

    match num {
        0 => {
            return res;
        },
        1 => {
            res.push(a);
        },
        2 => {
            res.push(a);
            res.push(b);
        },
        _ => {
            res.push(a);
            res.push(b);

            for _ in 2..num {
                let c = a + b;
                a = b;
                b = c;
                res.push(c);
            }
        },
    }

    return res;
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

    let res:Vec<i32> = fib(num);

    println!("{:?}", res);
}
