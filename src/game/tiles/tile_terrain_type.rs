use super::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TerrainType {
    // High altitude
    Mountains,
    ExtremeMountains,
    VolcanicMountains,
    // Cold tiles
    Icefields,
    Glacier,
    BorealForest,
    Tundra,
    // Hot tiles
    Jungle,
    TropicalForest,
    Desert,
    Savannah,
    // Temperate tiles
    Swamp,
    ConiferForest,
    TemperateForest,
    Plains,
}

impl TerrainType {
    pub fn spawn(tile_elements: &TileElements, tile_properties: &TileProperties) -> Self {
        select_terrain_type(&tile_elements, &tile_properties)
    }
}

fn select_terrain_type(
    tile_elements: &TileElements,
    tile_properties: &TileProperties,
) -> TerrainType {
    // High altitude
    if tile_properties.topography >= 8 {
        // it's a mega mountain!
        if tile_elements.elements_label == "EEE".to_string() {
            return TerrainType::ExtremeMountains;
        }
        if tile_properties.vulcanism >= 6 {
            return TerrainType::VolcanicMountains;
        }
        return TerrainType::Mountains;
    }

    // Hot tiles
    if tile_properties.temperature >= 8 {
        if tile_properties.humidity >= 7 && tile_properties.vegetation >= 7 {
            return TerrainType::Jungle;
        }
        if tile_properties.vegetation >= 6 {
            return TerrainType::TropicalForest;
        }
        if tile_properties.humidity <= 4 {
            return TerrainType::Desert;
        }

        return TerrainType::Savannah;
    }

    // Cold tiles
    if tile_properties.temperature <= 2 {
        if tile_properties.temperature == 0 {
            return TerrainType::Icefields;
        }
        if tile_properties.humidity >= 6 && tile_properties.topography >= 4 {
            return TerrainType::Glacier;
        }
        if tile_properties.vegetation >= 6 {
            return TerrainType::BorealForest;
        }

        return TerrainType::Tundra;
    }

    // Temperate tiles
    if tile_properties.humidity >= 7 && tile_properties.vegetation >= 7 {
        return TerrainType::Swamp;
    }
    if tile_properties.vegetation >= 6 {
        if tile_properties.topography >= 4 {
            return TerrainType::ConiferForest;
        }
        return TerrainType::TemperateForest;
    }

    TerrainType::Plains
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
