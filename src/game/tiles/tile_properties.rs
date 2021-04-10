use super::*;

#[derive(Debug)]
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

    // Next the tile will have its main characteristics rolled for,
    // with the upper and lower bounds (or at least the relative probabilities of such)
    pub fn spawn(tile_elements: &TileElements) -> Self {
        let topography: u8 = get_topography(&tile_elements);
        let vulcanism: u8 = rnjesus::rand_u8(0, 10);
        let temperature: u8 = get_temperature(&tile_elements, &topography);
        let humidity: u8 = rnjesus::rand_u8(0, 10);
        let vegetation: u8 = rnjesus::rand_u8(0, 10);

        let children: u8 = get_children(&tile_elements);
        let distance: u8 = get_distance(&tile_elements);

        TileProperties::new(
            topography,
            vulcanism,
            temperature,
            humidity,
            vegetation,
            children,
            distance,
        )
    }
}

fn get_topography(tile_elements: &TileElements) -> u8 {
    if tile_elements.has_element_n(&Element::Air, 3) {
        return 10;
    }
    if tile_elements.has_element_n(&Element::Air, 2) {
        return 9;
    }
    if tile_elements.has_element(&Element::Air) {
        return 7;
    }

    rnjesus::d10()
}

fn get_temperature(tile_elements: &TileElements, topography: &u8) -> u8 {
    if tile_elements.has_element_n(&Element::Fire, 3) {
        return 10;
    }
    if tile_elements.has_element_n(&Element::Fire, 2) {
        return 9;
    }
    rnjesus::rand_u8(1, *topography)
}

fn get_children(tile_elements: &TileElements) -> u8 {
    match &tile_elements.elements_label[..] {
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

fn get_distance(tile_elements: &TileElements) -> u8 {
    match &tile_elements.elements_label[..] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_spawn_tile_properties() {
        let tile_elements = TileElements::new([Element::Earth, Element::Earth, Element::Water]);
        let tile_properties = TileProperties::spawn(&tile_elements);

        assert_eq!(tile_properties.children, 9);
        assert_eq!(tile_properties.distance, 1);
    }
}
