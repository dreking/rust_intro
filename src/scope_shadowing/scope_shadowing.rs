pub fn scope_and_shadowing() {
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 789;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
    // println!("outside, b = {}", b);
}
