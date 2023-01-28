trait Animal {
    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name);
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name);
    }
}

enum Creature {
    Human(Human),
    Cat(Cat),
}

pub fn vector_multitypes() {
    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Human { name: "John" }));
    animals.push(Box::new(Cat { name: "Misty" }));

    for animal in animals {
        animal.talk();
    }

    // or use ENUMS
    let mut creatures: Vec<Creature> = Vec::new();
    creatures.push(Creature::Human(Human { name: "John" }));
    creatures.push(Creature::Cat(Cat { name: "Misty" }));

    for creature in creatures {
        match creature {
            Creature::Human(human) => human.talk(),
            Creature::Cat(cat) => cat.talk(),
        }
    }
}
