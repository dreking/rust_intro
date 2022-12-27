pub fn iterations() {
    let mut vec = vec![1, 2, 3, 4, 5];

    // iterate over values
    for i in &vec {
        println!("{}", *i);
    }

    for i in vec.iter() {
        println!("{}", i);
    }

    println!("{:?}", vec);

    // iterate over values and mutate them
    for i in &mut vec {
        *i += 50;
    }

    for i in vec.iter_mut() {
        *i += 50;
    }

    println!("{:?}", vec);

    // reverse iterate
    for i in vec.iter().rev() {
        println!("{}", i);
    }

    // iterate over values and their index
    for (index, value) in vec.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }

    // iterate over values and their index and mutate them
    for (index, value) in vec.iter_mut().enumerate() {
        *value += index as i32;
    }

    println!("{:?}", vec);

    // extend vector
    vec.extend(vec![6, 7, 8, 9, 10]);
    println!("{:?}", vec);

    // insert value at index
    vec.insert(0, 0);
    println!("{:?}", vec);

    // remove value at index
    vec.remove(0);
    println!("{:?}", vec);

    // remove last value
    vec.pop();
    println!("{:?}", vec);

    // remove first value
    vec.remove(0);
    println!("{:?}", vec);

    // remove all values
    vec.clear();
    println!("{:?}", vec);

    // check if vector is empty
    println!("{}", vec.is_empty());
}
