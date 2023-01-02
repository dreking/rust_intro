pub fn strings() {
    let s: &'static str = "Hello, world!";
    // let s_slice = "Hello, world!"; // same as above using &str

    // iterate over string
    for word in s.chars() {
        println!("{}", word);
    }

    // iterate over string and reverse
    for word in s.chars().rev() {
        println!("{}", word);
    }

    // get first character
    if let Some(first_char) = s.chars().nth(0) {
        println!("{}", first_char);
    }

    // string heap vector
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    // &str to String
    println!("{}", "hello world".to_string());
    println!("{}", String::from("hello world"));

    // String to &str
    println!("{}", String::from("hello world").as_str());
    // println!("{}", letters); // same as above

    // concatenate strings+str
    println!("{}", String::from("qwerty") + "abc");

    // remove character at index
    println!("{}", String::from("qwerty").remove(2));

    // replace substring
    println!("{}", String::from("qwerty").replace("qwe", "abc"));
}

pub fn formatting() {
    let name = "Leandre";
    let greetings = format!("Hello, {}!", name);
    println!("{}", greetings);

    let name = "Leandre";
    let rust = "Rust";
    let greetings = format!("Hello, {}! Welcome to {}!", name, rust);
    println!("{}", greetings);

    let name = "Leandre";
    let rust = "Rust";
    let greetings = format!("Hello, {0}! Welcome to {1}! {1} is awesome", name, rust);
    println!("{}", greetings);

    // named arguments
    let greetings = format!("Hello, {name}!", name = "Leandre");
    println!("{}", greetings);

    let greetings = format!(
        "Hello, {name}! Welcome to {lang}!",
        name = "Leandre",
        lang = "Rust"
    );
    println!("{}", greetings);

    let mixed = format!("{1} {} {0} {} {data}", "Damn!", "!!", data = "Rust");
    println!("{}", mixed);
}
