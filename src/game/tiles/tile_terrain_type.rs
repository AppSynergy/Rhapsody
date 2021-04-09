use super::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TerrainType {
    Desert,
    Jungle,
    Taiga,
    Mountains,
}

fn select_terrain_type(tile_elements: &TileElements, tile_properties: &TileProperties) -> TerrainType {
    if tile_elements.elements_label == "EEE".to_string() {
        return TerrainType::Mountains
    }

    match &tile_properties {
        TileProperties { temperature: t, .. } if t > &7 => TerrainType::Desert,
        TileProperties { humidity: h, .. } if h > &7 => TerrainType::Jungle,
        _ => TerrainType::Taiga,
    }
}

impl TerrainType {
    pub fn spawn(tile_elements: &TileElements, tile_properties: &TileProperties) -> Self {
        select_terrain_type(&tile_elements, &tile_properties)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_select_terrain_types() {
        let tile_elements = TileElements::new([Element::Earth, Element::Earth, Element::Earth]);
        let terrain_type = select_terrain_type(&tile_elements, &TileProperties::spawn(&tile_elements));

        assert_eq!(terrain_type, TerrainType::Mountains)
    }
}
