pub fn match_statement() {
    let country_code = 100000;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown", // ..= is a range that includes both end points
        _ => "Invalid",
    };

    println!("The country with code {} is {}", country_code, country);

    let x = true;

    let num = match x {
        true => "Yes",
        false => "No",
    };

    println!("num = {}", num);
}
