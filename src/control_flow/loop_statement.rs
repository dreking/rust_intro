pub fn while_loop() {
    let mut i = 0;
    println!("While loop: ");
    while i < 5 {
        println!("i = {}", i);
        i += 1;
    }
}

pub fn loop_infinite() {
    let mut i = 0;
    println!("Infinite loop: ");
    loop {
        println!("i = {}", i);
        i += 1;
        if i == 5 {
            break;
        }
    }
}

pub fn for_loop() {
    println!("For loop: ");
    for i in 0..5 {
        println!("i = {}", i);
    }

    // tracking index
    println!("For loop with index: ");
    for (index, value) in (5..10).enumerate() {
        println!("index = {}, value = {}", index, value);
    }
}
