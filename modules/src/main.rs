
mod plant {
    #[derive(Debug)]
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }
    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
    pub mod tree {


        pub mod pouet {
            pub fn pouet() {

            }
        }
        pub fn hello() {
            use self::pouet;
            pouet::pouet();
        }
    }
}
mod menu {

    pub use crate::plant::tree;

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub mod dessert {
        pub fn call() {

        }
    }
}

use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;
use std::{cmp::Ordering};
use std::io::{self, Write};

mod sound;

fn main() {
    let mut v = plant::Vegetable::new("concomber");
    v.name = String::from("carrot");
    println!("{:?}", v);
    //println!("print v.id {}", v.id);

    println!("{:?}", menu::Appetizer::Soup);

    menu::tree::hello();

    let mut map = HashMap::new();
    map.insert(1, 2);

    sound::instrument::clarinet();
    crate::sound::instrument::clarinet();
}