pub fn ownership() {
    let v = vec![1, 2, 3];
    let v2 = v;
    // println!("v[0] is: {}", v[0]); // error[E0382]: borrow of moved value: `v`
    println!("v2[0] is: {}", v2[0]); // 1

    let x = 5;
    let y = x;
    println!("x is: {}", x); // 5 - x is still valid because it is a primitive type and is stored on the stack and not the heap like v is above which is a vector and is stored on the heap
    println!("y is: {}", y); // 5
}
