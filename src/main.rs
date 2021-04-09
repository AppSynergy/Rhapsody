extern crate rand;

mod game;
mod level;

use game::tiles::Tile;

use level::Level;

fn main() {
    println!("Hello, rhapsody!");
    let level = Level::new(20, 20);
    println!("{}", level);

    for _ in 0..4 {
        let mut parent = Tile::spawn();
        println!("NODE:: {}", parent);

        for _ in 0..13 {
            let tile = Tile::propagate(&parent);
            println!("spawn: {}", tile);
            parent = tile;
        }
        println!("\n");
    }
}
