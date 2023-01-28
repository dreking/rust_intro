pub fn borrowing() {
    let v = vec![1, 2, 3];
    let v2 = &v;
    println!("v[0] is: {}", v[0]); // 1 - v is still valid because it is a primitive type and is stored on the stack and not the heap like v is above which is a vector and is stored on the heap
    println!("v2[0] is: {}", v2[0]); // 1 - v2 is a reference to v and is valid because it is a primitive type and is stored on the stack and not the heap like v is above which is a vector and is stored on the heap

    let mut a = 40;
    {
        let b = &mut a;
        *b += 2;
    }
    println!("a is: {}", a); // 42 - a is still valid because it is a primitive type and is stored on the stack and not the heap like v is above which is a vector and is stored on the heap
}
