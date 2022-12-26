pub fn if_statement() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else if number < 10 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    println!("The value of number is: {}", if condition { 5 } else { 6 });

    // if statements are expressions
    println!(
        "Number {}",
        if number < 5 {
            "is less than 5"
        } else if number < 10 {
            "is less than 10"
        } else {
            "is greater than or equal to 10"
        }
    );

    // Nested if statements
    println!(
        "Number {}",
        if number < 5 {
            if number < 2 {
                "is less than 2"
            } else {
                "is greater than or equal to 2"
            }
        } else if number < 10 {
            "is less than 10"
        } else {
            "is greater than or equal to 10"
        }
    )
}
