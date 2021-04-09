use super::tile_properties::TileProperties;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TerrainType {
    Desert,
    Jungle,
    Taiga,
}

fn select_terrain_type(tile_properties: &TileProperties) -> TerrainType {
    match &tile_properties {
        TileProperties { temperature: t, .. } if t > &7 => TerrainType::Desert,
        TileProperties { humidity: h, .. } if h > &7 => TerrainType::Jungle,
        _ => TerrainType::Taiga,
    }
}

impl TerrainType {
    pub fn spawn(tile_properties: &TileProperties) -> Self {
        select_terrain_type(&tile_properties)
    }
}
