// drop trait
struct Creature {
    name: String,
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("Dropping {}", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dropped", self.name);
    }
}

pub fn traits_drop() {
    let c = Creature::new("John");
    println!("Creature created");
    drop(c);
    println!("End of main");
}
