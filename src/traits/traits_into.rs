// into trait
struct Person {
    name: String,
}

impl Person {
    // fn new(name: impl Into<String>) -> Person {
    // fn new<S>(name: S) -> Person where S: Into<String> {
    fn new<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
}

pub fn traits_into() {
    let p = Person::new("John");
    println!("p.name = {}", p.name);
}
