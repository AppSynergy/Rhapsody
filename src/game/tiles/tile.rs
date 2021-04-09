use super::*;

pub struct Tile {
    pub terrain_type: TerrainType,
    pub elements: TileElements,
    pub properties: TileProperties,
}

impl Tile {
    pub fn new(
        terrain_type: TerrainType,
        elements: TileElements,
        properties: TileProperties,
    ) -> Self {
        Tile {
            terrain_type,
            elements,
            properties,
        }
    }

    pub fn spawn() -> Self {
        let tile_elements = TileElements::new([Element::Air, Element::Earth, Element::Fire]);
        let tile_properties = TileProperties::new(&tile_elements);
        let terrain_type = TerrainType::Taiga;

        Tile::new(terrain_type, tile_elements, tile_properties)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_spawn_new_tiles() {
        let tile = Tile::spawn();

        assert_eq!(tile.properties.vegetation, 0)
    }
}
