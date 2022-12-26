struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

pub fn struct_structure() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point coordinates: ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };

    let myline = Line { start: p, end: p2 };

    println!("line start: ({}, {})", myline.start.x, myline.start.y);
    println!("line end: ({}, {})", myline.end.x, myline.end.y);
}
