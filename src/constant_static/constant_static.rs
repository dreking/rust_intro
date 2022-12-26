const OK: i32 = 200;
static COUNTER: i32 = 0;
static mut COUNTER2: i32 = 0;

pub fn constant_and_static() {
    println!("OK = {}", OK);
    println!("COUNTER = {}", COUNTER);
    unsafe {
        // unsafe block is required to access static mut variables
        // This is because static mut variables can be accessed from multiple threads at the same time
        // and Rust can't guarantee that the access is safe in that case
        // https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#unsafe-rust-enables-unsafe-operations
        COUNTER2 += 1;
        println!("COUNTER2 = {}", COUNTER2);
    }
}
