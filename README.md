# Learning Rust by Example

This was originally the `rust-for-ts-devs` repo found at https://github.com/arendjr/rust-for-ts-devs by @arendjr

I renamed it because there isn't really anything that is specific to TS devs.
I also restructured and added a bit of new material.

While using this repo, it is recommended that you make heavy use of the

 [Rust standard library documentation](https://doc.rust-lang.org/stable/std/index.html)

 and also the

 [Rust Book](https://doc.rust-lang.org/stable/book/)

Other helpful links:  
- [A more comprehensive learning library](https://www.rust-lang.org/learn) -- The official documentation list  
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust) -- Look here for 3rd party libraries for your projects  


## How to use this repository

Just open this repository in VS Code and make sure you Rust Analyzer installed.

Like many Rust crates.  It is organized in a way that puts all functionality into a library.
Then the library is directly used by an included binary (in src/bin/main.rs)
This is a convention that makes it easier to write and run integration tests on your code.

The top-level module is `src/lib.rs`

There are 5 modules as directories under src,  they represent the chapters.
In each directory is a `mod.rs` file, this will give an overview of the files/examples in each chapter.

Chapters:

1. [Basic Types and Control Flow](src/one/mod.rs)
2. [Memory Management](src/two/mod.rs)
3. [Advanced Types](src/three/mod.rs)
4. [Containers and Iterators](src/four/mod.rs)
5. [Async IO](src/five/mod.rs)
