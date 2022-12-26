#![allow(dead_code)]

union IntOrFloat {
    i: i32,
    f: f32,
}

pub fn union_structure() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;
    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    let mut iof = IntOrFloat { f: 1.23 };
    iof.f = 4.56;
    let value = unsafe { iof.f };
    println!("iof.f = {}", value);

    // using match statement
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("i = 42");
            }
            // IntOrFloat { i } => {
            //     println!("i = {}", i);
            // }
            IntOrFloat { f } => {
                println!("f = {}", f);
            }
        }
    }
}
