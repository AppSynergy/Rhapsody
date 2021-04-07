// todo remove me!
#![allow(warnings)]

extern crate rand;

mod biome;
mod level;
mod node;

use level::Level;

fn main() {
    println!("Hello, rhapsody!");
    let level = Level::new(20, 20);
    println!("{}", level);
}
