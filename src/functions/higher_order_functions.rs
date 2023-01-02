pub fn iseven(n: u32) -> bool {
    n % 2 == 0
}

// functions that returns another function
pub fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    move |y| y > limit
}

pub fn higher_order_functions() {
    // sum of all even squares that are less than 500
    let limit = 500;
    let mut sum = 0;

    // generators
    for i in 0.. {
        let isq = i * i;

        if greater_than(limit)(isq) {
            break;
        } else if iseven(isq) {
            sum += isq;
        }
    }
    println!("{}", sum);

    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| iseven(*x))
        .fold(0, |sum, x| sum + x);
    println!("higher order function sum = {}", sum2);
}
