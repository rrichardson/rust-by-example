#![allow(clippy::comparison_chain)]
#![allow(clippy::new_without_default)]
#![allow(dead_code)]

use futures;

pub mod five;
pub mod four;
pub mod one;
pub mod three;
pub mod two;
pub mod six {
    pub mod another {
        pub fn wat() -> String {
            format!("this is six::another::wat")
        }
    }
}
