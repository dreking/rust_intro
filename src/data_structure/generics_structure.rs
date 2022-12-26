#![allow(dead_code)]

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

pub fn generics_structure() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer.x = {}, integer.y = {}", integer.x, integer.y);
    println!("float.x = {}, float.y = {}", float.x, float.y);

    let both_integer = Point2 { x: 5, y: 10.7 };
    let both_float = Point2 { x: 1.0, y: 4 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!(
        "both_integer.x = {}, both_integer.y = {}",
        both_integer.x, both_integer.y
    );
    println!(
        "both_float.x = {}, both_float.y = {}",
        both_float.x, both_float.y
    );
    println!(
        "integer_and_float.x = {}, integer_and_float.y = {}",
        integer_and_float.x, integer_and_float.y
    );

    // let p1 = Point { x: 5, y: 10 };
    // let p2 = Point { x: 1.0, y: 4.0 };
    // let myline = Line { start: p1, end: p2 }; // error[E0308]: mismatched types
    // println!(
    //     "myline.start.x = {}, myline.start.y = {}",
    //     myline.start.x, myline.start.y
    // );
}
