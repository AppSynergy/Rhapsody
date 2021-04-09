use super::*;

#[derive(Debug)]
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
        let tile_elements = TileElements::spawn();
        let tile_properties = TileProperties::spawn(&tile_elements);
        let terrain_type = TerrainType::spawn(&tile_elements, &tile_properties);

        Tile::new(terrain_type, tile_elements, tile_properties)
    }

    pub fn propagate(_tile: &Tile) -> Self {
        // TODO
        Tile::spawn()
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

    #[test]
    fn can_propagate() {
        let tile = Tile::spawn();
        Tile::propagate(&tile);
    }
}
