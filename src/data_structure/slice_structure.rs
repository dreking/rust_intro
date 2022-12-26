pub fn slice_structure() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: &[i32] = &xs[1..4];
    let zs: &[i32] = &xs;
    println!("ys = {:?}", ys);
    println!("zs = {:?}", zs);
}
