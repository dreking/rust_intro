use std::fmt::Debug;

trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> String;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: String,
}
struct Cat {
    name: String,
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human {
            name: name.to_string(),
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat {
            name: name.to_string(),
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}

pub fn traits() {
    // let person = Human {
    //     name: "John".to_string(),
    // };
    let person = Human::create("John");
    person.talk();

    // let cat = Cat {
    //     name: "Misty".to_string(),
    // };
    let cat = Cat::create("Misty");
    cat.talk();

    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum());
}
#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// fn print_info(shape: impl Shape + Debug) {
// fn print_info<T>(shape: T) where T: Shape + Debug, {
fn print_info<T: Shape + Debug>(shape: T) {
    println!("This shape is `{:?}`", shape);
    println!("This shape has an area of {}", shape.area());
}

pub fn traits_param() {
    let c = Circle { radius: 2.0 };
    let s = Square { side: 3.0 };
    print_info(c);
    print_info(s);
}
