/// Iterators are another crowning achievement of the Rust system
/// Modern programming languages are often a mishmash
/// of functional features built around collections
/// The Rust teams have put considerable effort into creating
/// a tremendously powerful Iterator toolset while maintaining perfect type safety.

/// The standard library for iterators lives in `std::iter` and the documentation can be found at
/// https://doc.rust-lang.org/stable/std/iter/
///
/// Learn more about them in the Rust Book:
/// https://doc.rust-lang.org/book/ch13-02-iterators.html
///
pub fn my_first_iteration(a: u32) -> u32 {
    let myarray = [0u32, 1, 2, 3, 4, 5];
    let myvec = vec![6u32, 7, 8, 9, 10];
    let generated = 0u32..100;

    let master = generated.chain(myvec).chain(myarray);
    let halfway = master.map(|num| num * 10);
    halfway.sum()
}

pub fn my_next_iteration(a: u32) -> u32 {
    let myarray = [0u32, 1, 2, 3, 4, 5];
    let myvec = vec![6u32, 7, 8, 9, 10];
    let generated = 0u32..100;

    let master = generated.chain(myvec).chain(myarray);
    let halfway = master.map(|num| num * 10);
    halfway.fold(0, |accum, x| accum + x)
}

pub fn returning_iterators(a: u32) -> impl Iterator<Item = u32> {
    let myarray = [0u32, 1, 2, 3, 4, 5];
    let myvec = vec![6u32, 7, 8, 9, 10];
    let generated = 0u32..100;

    let master = generated.chain(myvec).chain(myarray);
    master.map(|num| num * 10)
}

pub fn run_iterator(a: u32) {
    let it = returning_iterators(a);
    for x in it {
        println!("iter: {}", x);
    }
}
