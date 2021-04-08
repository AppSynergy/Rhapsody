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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_tiles() {
        let t = TerrainType::Taiga;
        let tp = TileProperties::new();
        let te = TileElements::new([Element::Air, Element::Earth, Element::Fire]);

        let tile = Tile::new(t, te, tp);

        assert_eq!(tile.properties.vegetation, 0)
    }
}
