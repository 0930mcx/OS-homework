#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const LEN: usize = 1000;

#[no_mangle]
fn main() -> i32 {
    let mut s = [0u32; LEN];
    let mut cur = 0usize;
    let mut prev = 1u32;
    s[cur] = prev;
    let mut next = 1u32;
    while next <= 1000 {
        let tmp = next;
        next += prev;
        prev = tmp;
        let next = if cur + 1 == LEN { 0 } else { cur + 1 };
        s[next] = prev;
        cur = next;
        println!("{}", s[cur]);
    }
    println!("Test fibonacci sequence OK!");
    0
}
