extern crate rand;

mod game;
mod level;
mod node;

use game::tiles::{TerrainType, Tile, TileElements, TileProperties, Element};

use level::Level;

fn main() {
    println!("Hello, rhapsody!");
    let level = Level::new(20, 20);
    println!("{}", level);

    let t = Tile::new(
        TerrainType::Taiga,
        TileElements::new([Element::Fire, Element::Fire, Element::Water]),
        TileProperties::new(),
    );
}
