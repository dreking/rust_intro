use std::{thread, time};

pub fn concurrency() {
    println!("Concurrency");
    thread::spawn(|| {
        println!("Hello from a thread!");
    });

    println!("Hello from the main thread!");
    for _ in 0..10 {
        thread::sleep(time::Duration::from_millis(500));
    }

    // join handle
    let handle = thread::spawn(|| "Hello from a thread!");

    handle.join().unwrap();
}
