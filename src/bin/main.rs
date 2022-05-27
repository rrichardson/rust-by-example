use rust_by_example::*;

fn main() {
    println!(
        "Result: {}",
        one::numbers::add_c(123918723981739, 12938712938712938),
    );

    println!("some async");
    five::future::take_1();
    five::future::take_2();
    five::future::exec_it();
}
