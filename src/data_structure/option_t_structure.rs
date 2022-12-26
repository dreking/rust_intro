pub fn option_t_structure() {
    let x = 3.0;
    let y = 2.0;

    let result = if y != 0.0 { Some(x / y) } else { None };

    // option type can be used in match statement
    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("Cannot divide {} by {}", x, y),
    }

    // option type can be used in if let statement
    if let Some(z) = result {
        println!("{}/{} = {}", x, y, z);
    } else {
        println!("Cannot divide {} by {}", x, y);
    }

    // option type can be used in while let statement
    while let Some(z) = result {
        println!("{}/{} = {}", x, y, z);
        break;
    }
}
