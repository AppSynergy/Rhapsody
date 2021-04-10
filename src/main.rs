extern crate rand;

mod game;
mod level;

use game::tiles::Tile;

use level::Level;

fn main() {
    println!("Hello, rhapsody!");
    let level = Level::new(20, 20);
    println!("{}", level);


    let mut parent = Tile::spawn();
    println!("SPAWNING..\n");
    println!("{}", parent);

    (0..parent.properties.children).for_each(|_| {
        let tile = Tile::propagate(&parent);
        println!("{}", tile);
        parent = tile;
    });
    println!("\n");

}
