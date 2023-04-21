extern crate rand;
use rand::Rng;

pub fn crates() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen();
    print!("Random number: {}", random_number)
}
