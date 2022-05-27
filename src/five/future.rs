/// async is a special keyword which restructures functions - under the hood -
/// to work with, and return futures.
///
/// any function decorated with async will implicitly return `Future<...>`
use futures::executor::{block_on, LocalPool};
use futures::task::LocalSpawnExt;

/// Futures require an executor in order to run.  This is because most async/futures are oriented around I/O
/// So *something* needs to be watching sockets to see when data is ready, and when it is, it will wake up
/// and activate the futures waiting on the data.  The executor keeps all futures, waiters, and wakers organized.
///
/// Block-On is a very simple executor that we'll use for now in place of more advanced things.

pub async fn hello() {
    println!("hello, from the past!");
}

/// The most important thing to note about Futures is that they don't run until
/// executed  (using `await`, or something else)

pub fn take_1() {
    let future = hello(); // Nothing is printed
    println!("We are here");
    block_on(future); // `future` is run and "hello, world!" is printed
}

///
/// This becomes more apparent when we run multiple futures

async fn learn_rust() {
    println!("I know rust fu");
}

async fn prove_it() {
    println!("show me.");
}

async fn rust_fu() {
    println!("pub async fn rust_fu()");
}

pub fn take_2() {
    let step1 = learn_rust();
    let step2 = prove_it();
    let step3 = rust_fu();
    println!("starting...");
    let mut pool = LocalPool::new();
    let spawner = pool.spawner();
    spawner.spawn_local(step1).unwrap();
    spawner.spawn_local(step2).unwrap();
    spawner.spawn_local(step3).unwrap();

    pool.run_until_stalled();
}

/// The above example doesn't guarantee order though..
/// If we want these functions to run in order, then we have two options:
/// We can use a `combinator` like the [then](https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.then) method
/// Or we can take advantage of Rust's helpful syntax and use `await`

pub async fn run_all() {
    learn_rust().await;
    prove_it().await;
    rust_fu().await;
}

/// Block on
pub fn exec_it() {
    block_on(run_all())
}
