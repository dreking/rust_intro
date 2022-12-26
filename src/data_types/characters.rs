use std::mem;

pub fn characters() {
    // char: 4 bytes in size and can hold one Unicode Scalar Value
    let a = 'a';
    println!("a = {}, size = {} bytes", a, mem::size_of_val(&a));

    // str: 16 bytes in size and can hold a string of Unicode Scalar Values
    let b = "Hello, world!";
    println!("b = {}, size = {} bytes", b, mem::size_of_val(&b));
}
