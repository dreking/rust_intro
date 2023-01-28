trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("String: {}", *self)
    }
}

// monomorphisation
fn print_it<T: Printable>(z: T) {
    println!("{}", z.format());
}

pub fn static_dispatch() {
    let a = 1;
    let b = "Hello".to_string();
    println!("{}", a.format());
    println!("{}", b.format());

    print_it(a);
    print_it(b);
}

// monomorphisation
fn print_it2(z: &dyn Printable) {
    println!("{}", z.format());
}

pub fn dynamic_dispatch() {
    let a = 1;
    let b = "Hello".to_string();
    println!("{}", a.format());
    println!("{}", b.format());

    print_it2(&a);
    print_it2(&b);
}
