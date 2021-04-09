use super::rnjesus;
use super::{Element, ThreeElements, TileElements};

pub struct TileProperties {
    // Environmental properties
    pub topography: u8,
    pub vulcanism: u8,
    pub temperature: u8,
    pub humidity: u8,
    pub vegetation: u8,
    // Structural properties
    children: u8,
    distance: u8,
}

fn get_topography(elements: &ThreeElements) -> u8 {
    match elements {
        [Element::Fire, Element::Fire, Element::Fire] => 7,
        _ => 0,
    }
}

fn get_children(elements_label: &String) -> u8 {
    match &elements_label[..] {
        "AAA" => rnjesus::rand_u8(9, 12),

        "FFF" => rnjesus::rand_u8(12, 40),
        "AFF" => rnjesus::rand_u8(9, 20),
        "FFE" => rnjesus::rand_u8(9, 20),
        "FFW" => rnjesus::rand_u8(9, 16),

        "EEE" => 24,

        "WWW" => 40,
        "AWW" => 16,
        "EWW" => 16,
        "FWW" => 16,

        _ => 9,
    }
}

fn get_distance(elements_label: &String) -> u8 {
    match &elements_label[..] {
        "AAA" => rnjesus::rand_u8(2, 4),
        "AAE" => rnjesus::rand_u8(2, 3),
        "AAF" => rnjesus::rand_u8(2, 3),
        "AAW" => rnjesus::rand_u8(2, 3),

        "FFF" => rnjesus::rand_u8(1, 3),
        "AFF" => rnjesus::rand_u8(0, 2),
        "FFE" => rnjesus::rand_u8(0, 2),
        "FFW" => rnjesus::rand_u8(0, 2),

        "EEE" => 2,
        "AEE" => 1,
        "EFE" => 1,
        "EEW" => 1,

        "WWW" => 3,
        "AWW" => 2,
        "EWW" => 2,
        "FWW" => 2,

        _ => 0,
    }
}

impl TileProperties {
    pub fn new(
        topography: u8,
        vulcanism: u8,
        temperature: u8,
        humidity: u8,
        vegetation: u8,
        children: u8,
        distance: u8,
    ) -> Self {
        TileProperties {
            topography,
            vulcanism,
            temperature,
            humidity,
            vegetation,
            children,
            distance,
        }
    }

    pub fn spawn(tile_elements: &TileElements) -> Self {
        let label = &tile_elements.elements_label;
        let elements = &tile_elements.elements;

        let topography = get_topography(elements);

        let children = get_children(label);
        let distance = get_distance(label);

        TileProperties::new(topography, 0, 0, 0, 0, children, distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_tile_properties() {
        let tile_elements = TileElements::new([Element::Earth, Element::Earth, Element::Water]);
        let tile_properties = TileProperties::spawn(&tile_elements);

        assert_eq!(tile_properties.vegetation, 0);
        assert_eq!(tile_properties.children, 9);
        assert_eq!(tile_properties.distance, 1);
    }
}
