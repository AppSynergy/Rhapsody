use super::Element;
use super::TileElements;

pub struct TileProperties {
    pub topography: u8,
    pub vulcanism: u8,
    pub temperature: u8,
    pub humidity: u8,
    pub vegetation: u8,
}

impl TileProperties {
    pub fn new(tile_elements: &TileElements) -> Self {
        TileProperties {
            vegetation: 0,
            humidity: 0,
            temperature: 0,
            vulcanism: 0,
            topography: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_tile_properties() {
        let tile_elements = TileElements::new([Element::Earth, Element::Earth, Element::Water]);
        let tile_properties = TileProperties::new(&tile_elements);
        assert_eq!(tile_properties.vegetation, 0)
    }
}
