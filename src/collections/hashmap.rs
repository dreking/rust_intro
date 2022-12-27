use std::collections::HashMap;

pub fn hashmap() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);
    shapes.insert(String::from("pentagon"), 5);
    println!("{:?}", shapes);

    for (key, value) in &shapes {
        println!("{}: {}", key, value);
    }

    // override existing value
    shapes.insert(String::from("triangle"), 5);
    println!("{:?}", shapes);

    // only insert if the key has no value
    shapes.entry(String::from("circle")).or_insert(10);
    println!("{:?}", shapes);

    // update key value if key is found
    {
        let circle = shapes.entry(String::from("circle")).or_insert(0);
        *circle += 1;
    }
    println!("{:?}", shapes);
}
