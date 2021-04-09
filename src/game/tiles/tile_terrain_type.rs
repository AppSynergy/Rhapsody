use super::tile_properties::TileProperties;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TerrainType {
    Desert,
    Plains,
    Grassland,
    Forest,
    Taiga,
    Jungle,
    Savannah,
    Tundra,
}

fn select_terrain_type(_tile_properties: TileProperties) -> TerrainType {
    TerrainType::Desert
}
