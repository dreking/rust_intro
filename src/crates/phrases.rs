extern crate phrases;
use phrases::greetings::english;
use phrases::greetings::french;

pub fn phrases() {
    println!("Hello {}", phrases::greetings::english::hello());
    println!("Hello {}", phrases::greetings::french::hello());
    println!("Hello {}", phrases::greetings::english::goodbye());
    println!("Hello {}", phrases::greetings::french::goodbye());

    println!("Hello {}", english::hello());
    println!("Hello {}", french::hello());
    println!("Hello {}", english::goodbye());
    println!("Hello {}", french::goodbye());
}
