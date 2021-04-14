use super::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TerrainType {
    Tundra,
    BorealForest,
    Plains,
    LushForest,
    TemperateForest,
    Shrubland,
    Prarie,
    Grassland,
    Jungle,
    TropicalForest,
    Savannah,
    Desert,
}

impl TerrainType {
    pub fn spawn(tile_elements: &TileElements, tile_properties: &TileProperties) -> Self {
        select_terrain_type(&tile_elements, &tile_properties)
    }
}

// Based on these rolls, a type is inferred
fn select_terrain_type(
    _tile_elements: &TileElements,
    tile_properties: &TileProperties,
) -> TerrainType {
    // Estimate general biome based on climate and humidity
    // https://upload.wikimedia.org/wikipedia/commons/6/68/Climate_influence_on_terrestrial_biome.svg
    let t = tile_properties.climate;
    let h = tile_properties.humidity;

    let wooded = |wooded_type: TerrainType, unwooded_type: TerrainType| {
        if tile_properties.vegetation >= 6 {
            return wooded_type;
        }
        return unwooded_type;
    };

    if t <= 3 {
        return TerrainType::Tundra;
    }
    if t <= 5 {
        if h >= 3 {
            return TerrainType::BorealForest;
        } else {
            return TerrainType::Plains;
        }
    } else if t <= 8 {
        if h >= 6 {
            return TerrainType::LushForest;
        } else if h >= 4 {
            return wooded(TerrainType::TemperateForest, TerrainType::Grassland);
        } else if h >= 2 {
            return wooded(TerrainType::Shrubland, TerrainType::Prarie);
        } else {
            return TerrainType::Grassland;
        }
    } else {
        if h >= 7 {
            return TerrainType::Jungle;
        } else if h >= 3 {
            return wooded(TerrainType::TropicalForest, TerrainType::Savannah);
        } else {
            return TerrainType::Desert;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_select_terrain_types() {
        let tile_elements = TileElements::new([Element::Earth, Element::Earth, Element::Earth]);
        select_terrain_type(&tile_elements, &TileProperties::spawn(&tile_elements));
    }
}
