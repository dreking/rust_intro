pub fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a has {} elements", a.len());
    println!("a[0] = {}", a[0]);
    println!("{:?}", a);

    // usize vs isize
    let index = 0;
    a[index] = 7;
    println!("a[{}] = {}", index, a[index]);

    // This will cause panic, because the index is out of bounds
    // println!("Get element at index 6, {}", a.get(6).unwrap());

    // This will not cause panic, because the index is out of bounds
    match a.get(6) {
        Some(x) => println!("Get element at index 6, {}", x),
        None => println!("There is no element at index 6"),
    }

    // iteration
    for x in &a {
        println!("{}", x);
    }

    // pop last element
    println!("popped last element, a = {:?}", a.pop());
    // pop all elements
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
    println!("{:?}", a);
}
