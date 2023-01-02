pub fn fnarguments(x: i32) {
    println!("The value of x is: {}", x);
}

pub fn increase(x: &mut i32) {
    *x += 1;
    println!("The value of x is: {}", x);
}

pub fn product(x: i32, y: i32) -> i32 {
    println!("The product is: {}", x * y);
    x * y
    // return x * y;
}
