pub fn tuple_structure() {
    let pair = (1, true);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // pair contains 1 and true

    // tuples can be destructured to create bindings
    let (integer, boolean) = pair;
    println!("pair contains {:?} and {:?}", integer, boolean);
    // pair contains 1 and true

    // tuples are printable
    println!("pair is {:?}", pair);
    // pair is (1, true)

    // addition and multiplication are defined for tuples
    let x = 2;
    let y = 3;
    let response = (x + y, x * y);
    println!("{:?}", response);
    println!("{} + {} = {}", x, y, response.0);
    println!("{} * {} = {}", x, y, response.1);
    let response2 = (x + y, x * y);

    let combined = (response, response2);
    println!("{:?}", combined);
    let ((a, b), (c, d)) = combined;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}
