use std::collections::HashSet;

pub fn hashset() {
    let mut greeks = HashSet::new();
    greeks.insert("alpha");
    greeks.insert("beta");
    println!("{:?}", greeks);

    // add existing value
    greeks.insert("alpha");
    println!("{:?}", greeks);

    // add new value
    let isadded = greeks.insert("gamma");
    println!("{:?}", isadded);
    println!("{:?}", greeks);

    // check if value exists
    if !greeks.contains("gamma") {
        println!("gamma is not in the set");
    }

    // remove value
    let isremoved = greeks.remove("alpha");
    println!("{:?}", isremoved);
    println!("{:?}", greeks);

    // iterate over values
    for greek in &greeks {
        println!("{}", greek);
    }

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // is a subset
    println!(
        "is {:?} a subset of {:?}, {:?}",
        _1_5,
        _1_10,
        _1_5.is_subset(&_1_10)
    );

    // disjoint = no common elements
    println!(
        "is {:?} disjpint of {:?}, {:?}",
        _1_5,
        _6_10,
        _1_5.is_disjoint(&_6_10)
    );

    // union
    println!(
        "union of {:?} and {:?} is {:?}",
        _1_5,
        _6_10,
        _1_5.union(&_6_10)
    );

    // difference
    println!(
        "difference of {:?} and {:?} is {:?}",
        _1_10,
        _2_8,
        _1_10.difference(&_2_8)
    );
}
