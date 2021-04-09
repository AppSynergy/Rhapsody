extern crate rand;

mod game;
mod level;

use game::tiles::Tile;

use level::Level;

fn main() {
    println!("Hello, rhapsody!");
    let level = Level::new(20, 20);
    println!("{}", level);

    let tile = Tile::spawn();
    println!("{:?}", tile)

}
