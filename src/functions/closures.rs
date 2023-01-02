pub fn closures() {
    let add_one = |x: i32| -> i32 { x + 1 };
    let a = 6;
    println!("{} + 1 = {}", a, add_one(a));

    let add_two = |x| {
        let mut result: i32 = x;
        result += 1;
        result += 1;
        result
    };
    println!("{} + 2 = {}", a, add_two(a));

    let add_three = |x: &mut i32| *x += 3;
    let mut a = 3;
    add_three(&mut a);
    println!("{}", a);
}
