fn main() {
    println!("Hello, world!");
}

use std::collections::HashMap;

struct Tile {
    map: HashMap<String, String>
}

impl Tile {
    fn new(map: HashMap<String, String>) -> Self { Self { map } }


}
