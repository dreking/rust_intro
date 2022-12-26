use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    return Point { x: 0.0, y: 0.0 };
}

pub fn stack_and_heap() {
    let p1 = origin(); // stack allocated
    let p2 = Box::new(origin()); // heap allocated

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2; // p3 is a copy of the data pointed to by p2
    println!("{}", p3.x);
    println!("{}", p3.y);
}
