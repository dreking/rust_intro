pub fn array_structure() {
    let a = [1, 2, 3, 4, 5];

    // print array elements
    println!("a has {} elements", a.len());
    println!("first element of the array: {}", a[0]);
    println!("{:?}", a);

    // print array elements using while loop
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // print array elements using for loop
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // compare array elements
    if a == [1, 2, 3, 4, 5] {
        println!("a is equal to [1,2,3,4,5]");
    }

    // array of 10 elements, all initialized to 1
    let b = [1u16; 10]; // ten 1s
    println!("{:?}", b);

    // multi-dimensional array
    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    println!("{:?}", mtx);

    // print multi-dimensional array elements
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if j == 0 {
                print!("| ");
            }
            print!("{} ", mtx[i][j]);
            if j == mtx[i].len() - 1 {
                println!("|");
            }
        }
    }

    // array out of bounds
    // println!("{}", a[10]);
}
