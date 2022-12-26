pub fn pattern_matching() {
    for x in 0..10 {
        let y = match x {
            0 => "zero",
            1 | 2 => "one or two",
            7..=9 => "three to nine",
            _ if x % 2 == 0 => "even",
            _ => "many",
        };
        println!("{}: {}", x, y);
    }

    let point = (3, 5);
    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (x, y) => println!("({}, {})", x, y),
    }
}
