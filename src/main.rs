extern crate rand;

mod game;
mod level;

use game::tiles::Tile;

use level::Level;

fn main() {
    println!("Hello, rhapsody!");
    let level = Level::new(20, 20);
    println!("{}", level);

    let tile1 = Tile::spawn();
    println!("{:?}", tile1);

    let tile2 = Tile::propagate(&tile1);
    println!("{:?}", tile2);

    let tile3 = Tile::propagate(&tile1);
    println!("{:?}", tile3);

}
