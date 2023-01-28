pub fn reference_counting() {
    use std::rc::Rc; // Rc is not thread safe and is used for single threaded applications
    let r1 = Rc::new(5);
    let r2 = r1.clone();
    println!("r1 is: {}", r1);
    println!("r2 is: {}", r2);

    println!("reference count is: {}", Rc::strong_count(&r1));
    {
        let r3 = r1.clone();
        println!("r1 is: {}", r3);
    }
    println!("reference count is: {}", Rc::strong_count(&r1));

    use std::sync::Arc; // Arc is thread safe and is used for multi threaded applications
    let r1 = Arc::new(5);
    let r2 = r1.clone();
    println!("r1 is: {}", r1);
    println!("r2 is: {}", r2);

    println!("reference count is: {}", Arc::strong_count(&r1));
    {
        let r3 = r1.clone();
        println!("r1 is: {}", r3);
    }
    println!("reference count is: {}", Arc::strong_count(&r1));

    // mutex is used to allow only one thread to access a resource at a time
    use std::sync::Mutex;
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m is: {:?}", m);
}
