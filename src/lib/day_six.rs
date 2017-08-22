use std::io;

pub fn loop_mult() {

    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Failed to read input");

    let mut n = 1;
    let num :i32 = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => 0
    };

    while n <= 10 {
        println!("{} x {} = {}", num, n, num * n);
        n += 1;
    }
}
