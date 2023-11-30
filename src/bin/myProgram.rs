#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;

const MAX_VALUE: u32 = 200;

#[no_mangle]
fn main() -> i32 {
    let mut fib = [0u32; 2];
    fib[1] = 1;
    println!("My program is to show the Fibonacci Sequence.");
    println!("Fibonacci Sequence:");

    while fib[1] <= MAX_VALUE {
        println!("{}", fib[1]);
        let next = fib[0] + fib[1];
        fib[0] = fib[1];
        fib[1] = next;
    }

    0
}
