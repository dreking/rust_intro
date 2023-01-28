struct Person {
    name: String,
}

impl Person {
    fn get_ref_name(&self) -> &String {
        &self.name
    }
}

struct Company<'a> {
    name: String,
    ceo: &'a Person,
}

struct Person1 {
    name: &'static str,
}

impl Person1 {
    fn talk(&self) -> &'static str {
        self.name
    }
}

pub fn lifetime() {
    let p = Person {
        name: String::from("Steve"),
    };
    let c = Company {
        name: String::from("Apple"),
        ceo: &p,
    };
    println!("{} is the CEO of {}", c.ceo.name, c.name);

    // let mut z: &String;
    // {
    //     let p = Person {
    //         name: String::from("Steve"),
    //     };
    //     let c = Company {
    //         name: String::from("Apple"),
    //         ceo: &p, // error[E0597]: `p` does not live long enough
    //     };
    //     z = c.ceo.get_ref_name();
    // }
    // println!("z is: {}", z);

    let p = Person {
        name: String::from("Steve"),
    };
    let c = Company {
        name: String::from("Apple"),
        ceo: &p,
    };
    println!("{} is the CEO of {}", c.ceo.get_ref_name(), c.name);

    let p = Person1 { name: "Steve" };
    println!("{} is talking", p.talk());
}
