use std::f64::consts::PI;

pub fn operators() {
    let mut a = 2 + 3 * 4;
    println!("a = {}", a);

    a += 1;
    println!("a = {}", a);

    a -= 2;
    println!("a = {}", a);

    a *= 2;
    println!("a = {}", a);

    a /= 2;
    println!("a = {}", a);

    a %= 3;
    println!("a = {}", a);

    // a = a ^ 3;
    let a_cubed = i32::pow(12, 3);
    println!("a_cubed: {} ^ {} = {}", 12, 3, a_cubed);

    let b = 2.5;
    // power of 3, raise to the power of an integer
    let b_cubed = f64::powi(b, 3);
    // power of PI, raise to the power of a float
    let b_to_pi = f64::powf(b, PI);
    println!("b = {}, b_cubed = {}, b_to_pi = {}", b, b_cubed, b_to_pi);

    // bitwise operators, only available for integers
    // | OR, & AND, ^ XOR, ! NOR
    let c = 1 | 2; // 01 OR 10 = 11 == 3_10
    println!("1 | 2 = {}", c);
    println!("1 & 2 = {}", 1 & 2);
    println!("1 ^ 2 = {}", 1 ^ 2);
    println!("!3 = {}", !3);

    // shift operators
    // << left shift, >> right shift
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);
    println!("1024 >> 2 = {}", 1024 >> 3);

    // logical operators
    // >, <, >=, <=, ==, !=
    let pi_less_4 = PI < 4.0; // true
    println!("pi_less_4 = {}", pi_less_4);
}
