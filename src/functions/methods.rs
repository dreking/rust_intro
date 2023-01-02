struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another static method, taking two arguments:
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    // len() is an instance method
    // &self is sugar for self: &Self, where Self is the type of the caller object.
    // In this case Self = Point
    fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn methods() {
    let p1 = Point::origin();
    let p2 = Point::new(5.0, 6.0);
    let len = p2.len();
    println!("p1 is at: ({}, {})", p1.x, p1.y);
    println!("The length of p2 is: {}", len);
}
