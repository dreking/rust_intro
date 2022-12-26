use std::mem;

pub fn booleans() {
    // bool: 1 byte in size and can hold a true or false value
    let a = true;
    println!("a = {}, size = {} bytes", a, mem::size_of_val(&a));
}
