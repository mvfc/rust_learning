use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");
    let x: u32 = input.trim().parse().expect("Not a number");
    println!("{}", fib(x));
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    };
    return fib(n-1)+fib(n-2);
}
