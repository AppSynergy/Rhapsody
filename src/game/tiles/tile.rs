use super::*;

#[derive(Debug)]
pub struct Tile {
    pub terrain_type: TerrainType,
    pub elements: TileElements,
    pub properties: TileProperties,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "╭──────────────────────────────────╮\n")?;
        write!(
            f,
            "│ {:>7} {} {:>18} │\n{}\n",
            self.elements,
            self.elements.elements_label,
            format!("{:?}", self.terrain_type),
            self.properties
        )
    }
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

    pub fn propagate(tile: &Tile) -> Self {
        // Depending on the title's elemental characteristics, it multiplies/propogates in different ways:
        let tile_elements = TileElements::propagate(&tile.elements);
        // TODO convert to propagate??
        let tile_properties = TileProperties::spawn(&&tile_elements);
        let terrain_type = TerrainType::spawn(&tile_elements, &tile_properties);

        Tile::new(terrain_type, tile_elements, tile_properties)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_spawn_new_tiles() {
        Tile::spawn();
    }

    #[test]
    fn can_propagate() {
        let tile = Tile::spawn();
        Tile::propagate(&tile);
    }
}
