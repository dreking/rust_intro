use std::mem;

pub fn integers() {
    // Unsigned, immutable and data type inferred variables
    let a: u8 = 123;
    println!("a = {a}");

    // Signed, mutable and data type inferred variables
    let mut b: i8 = 0;
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    // Signed, mutable and non inferred data type variables
    let c = 123456789; // i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    // usize: assign unisigned integer based on the platform
    // isize: assign signed integer based on the platform
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );
}
