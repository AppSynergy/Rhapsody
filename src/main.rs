extern crate rand;

mod game;
mod level;
mod node;

use game::tiles::Tile;

use level::Level;

fn main() {
    println!("Hello, rhapsody!");
    let level = Level::new(20, 20);
    println!("{}", level);

    let _t = Tile::spawn();
}
