use std::mem;

pub fn floats() {
    // f32: 4 bytes in size and can hold a single precision floating point number (32 bits)
    let a: f32 = 2.5;
    println!("a = {}, size = {} bytes", a, mem::size_of_val(&a));

    // f64: 8 bytes in size and can hold a double precision floating point number (64 bits)
    let b = 2.5; // f64
    println!("b = {}, size = {} bytes", b, mem::size_of_val(&b));
}
