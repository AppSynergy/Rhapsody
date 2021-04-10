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
    pub children: u8,
    pub distance: u8,
}

impl fmt::Display for TileProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "╞══════════════════════════════════╡\n")?;
        write!(
            f,
            "│{:>2} ⛰  │{:>2} 🌋 │{:>2} 🏜  │{:>2} 🌧  │{:>2} 🌱 │",
            self.topography.to_string(),
            self.vulcanism,
            self.temperature,
            self.humidity,
            self.vegetation
        )
        .ok();
        write!(f, "\n╰──────────────────────────────────╯")?;

        Ok(())
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

    // Next the tile will have its main characteristics rolled for,
    // with the upper and lower bounds (or at least the relative probabilities of such)
    pub fn spawn(tile_elements: &TileElements) -> Self {
        // Environmental properties
        let topography: u8 = get_topography(&tile_elements);
        let vulcanism: u8 = get_vulcanism(&tile_elements, &topography);
        let temperature: u8 = get_temperature(&tile_elements, &topography);
        let humidity: u8 = get_humidity(&tile_elements, &temperature);
        let vegetation: u8 = get_vegetation(&tile_elements, &humidity);
        // Structural properties
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

    pub fn propagate(tile_elements: &TileElements, _tile_properties: &TileProperties) -> Self {
        Self::spawn(tile_elements)
    }
}

fn get_topography(tile_elements: &TileElements) -> u8 {
    let mut max_topography: u8 = 7;
    if tile_elements.is_triple_of(&Element::Air) {
        return 0;
    } else if tile_elements.is_triple_of(&Element::Earth) {
        return 10;
    }
    if tile_elements.has(&Element::Air) {
        max_topography = 10;
    }

    rnjesus::dx(max_topography, 10)
}

fn get_vulcanism(tile_elements: &TileElements, topography: &u8) -> u8 {
    let mut max_vulcanism: u8 = *topography + 1;
    if tile_elements.is_triple_of(&Element::Earth) {
        return 10;
    }
    if tile_elements.is_double_of(&Element::Earth) {
        max_vulcanism = 10;
    }

    rnjesus::dx(max_vulcanism, 10)
}

fn get_temperature(tile_elements: &TileElements, topography: &u8) -> u8 {
    let mut max_temperature: u8 = 12 - *topography;
    if tile_elements.is_triple_of(&Element::Fire) {
        return 10;
    }
    if tile_elements.is_double_of(&Element::Fire) {
        max_temperature += 3;
    }
    if tile_elements.has(&Element::Fire) {
        max_temperature += 2;
    } else if tile_elements.has(&Element::Air) {
        max_temperature -= 1;
    }
    rnjesus::dx(max_temperature, 10)
}

fn get_humidity(tile_elements: &TileElements, temperature: &u8) -> u8 {
    let mut max_humidity: u8 = *temperature + 3;
    if tile_elements.is_triple_of(&Element::Fire) {
        return 1;
    }
    if tile_elements.is_triple_of(&Element::Water) {
        return 10;
    }
    if tile_elements.is_double_of(&Element::Water) {
        max_humidity += 3;
    }
    if tile_elements.has(&Element::Water) {
        max_humidity += 2;
    }
    rnjesus::dx(max_humidity, 10)
}

fn get_vegetation(tile_elements: &TileElements, humidity: &u8) -> u8 {
    let mut max_vegetation = *humidity + 2;
    if tile_elements.is_single() {
        max_vegetation += 2
    }
    rnjesus::dx(max_vegetation, 10)
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
