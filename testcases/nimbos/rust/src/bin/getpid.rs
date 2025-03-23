#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{getpid};

#[no_mangle]
pub fn main() -> isize {
    let pid = getpid();
    
    println!("My pid is: {}", pid);
    pid
}
