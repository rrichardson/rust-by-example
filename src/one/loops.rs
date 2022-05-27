/// loops come in three flavors in Rust
/// for,  while, and loop
///
use std::collections::HashMap;

/// `for`
/// `for <variable> in <iterator>`
/// iterates over an iterator, assigning at least 1 variable to the current iteration
pub fn forsooth() {
    for x in 0..10 {
        println!("{}", x);
    }
}

pub fn iter_map() {
    let mymap = HashMap::from([("a", 1_u32), ("b", 2_u32), ("c", 3_u32)]);
    for (key, val) in mymap.iter() {
        println!("{} = {}", key, val);
    }
}
/// This might look like a counter but it isn't.  `..` is a range operator. It actually takes a beginning and an end and makes an iterator.
/// There is no javascript or C `for` equivalent.  If you want to muck about with incrementing indexes in arrays, do it somewhere else.

/// You might notice that 0..10  actually counts from 0 to 9.  So it is an exclusive range.  
/// If you want an inclusive range, use 0..=10

/// `while`
/// `while <condition>` runs a loop for as long as <condition> is true
///
pub fn loop_a_lot() {
    let mut i = 0;

    while i < 500 {
        println!("we are here: {}", i);
        i += 1;
    }
}

pub fn loop_less() {
    let mut r = 0..300;

    while let Some(num) = r.next() {
        println!("I wish I were a for loop {}", num);
    }
}

/// `loop`
/// As the name implies, this simply runs forever.  
/// This is mostly for the main driver of programs, or event processing loops
/// that are meant to run until.. well.. they're not.
///
pub fn loop_forever() {
    let mut i = 0;

    loop {
        i += 1;
        if i > 1000 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        println!("odd number: {}", i);
    }
}
