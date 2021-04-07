extern crate rand;

mod biome;
mod level;
mod node;

fn main() {
    let tile = Tile::new(HashMap::new());
    println!("Hello, world!");
    println!("{}", tile.map.len());
    Level::new(200, 200);
}

use std::collections::HashMap;



use level::Level;struct Tile {
    map: HashMap<String, String>
}

impl Tile {
    fn new(map: HashMap<String, String>) -> Self { Self { map } }


}


#[cfg(test)]
mod tests {
    use crate::Tile;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let tile = Tile::new(HashMap::new());
        assert_eq!(0, tile.map.len());
    }
}
