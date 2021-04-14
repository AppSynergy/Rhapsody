use super::*;

#[derive(Debug)]
pub struct TileProperties {
    // Environmental properties
    pub topography: u8,
    pub vulcanism: u8,
    pub climate: u8,
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
            self.climate,
            self.humidity,
            self.vegetation
        )
        .ok();
        write!(f, "\n╰──────────────────────────────────╯")?;

        Ok(())
    }
}

pub enum SeaType {
    None,
    Ocean,
    LavaOcean,
}

impl TileProperties {
    pub fn new(
        topography: u8,
        vulcanism: u8,
        climate: u8,
        humidity: u8,
        vegetation: u8,
        children: u8,
        distance: u8,
    ) -> Self {
        TileProperties {
            topography,
            vulcanism,
            climate,
            humidity,
            vegetation,
            children,
            distance,
        }
    }

    // Next the tile will have its main characteristics rolled for,
    // with the upper and lower bounds (or at least the relative probabilities of such)
    pub fn spawn(tile_elements: &TileElements) -> Self {
        // Primary environmental properties
        let topography: u8 = get_topography(&tile_elements);
        let vulcanism: u8 = get_vulcanism(&tile_elements);
        let climate: u8 = get_climate(&tile_elements);
        let humidity: u8 = get_humidity(&tile_elements);
        // Secondary environmental properties
        let _sea_type: SeaType = get_sea_type(&vulcanism, &humidity);
        let vegetation: u8 = get_vegetation(&climate, &humidity);
        // Structural properties
        let children: u8 = get_children(&tile_elements);
        let distance: u8 = get_distance(&tile_elements);

        TileProperties::new(
            topography, vulcanism, climate, humidity, vegetation, children, distance,
        )
    }

    pub fn propagate(tile_elements: &TileElements, _tile_properties: &TileProperties) -> Self {
        Self::spawn(tile_elements)
    }
}

fn get_topography(tile_elements: &TileElements) -> u8 {
    // air vs earth
    let mut p = 0.5;
    for element in tile_elements.elements.iter() {
        match element {
            Element::Air => p += 0.1,
            Element::Earth => p -= 0.1,
            _ => {}
        }
    }

    rnjesus::binom(10, p)
}

fn get_vulcanism(tile_elements: &TileElements) -> u8 {
    // fire vs water
    let mut p = 0.5;
    for element in tile_elements.elements.iter() {
        match element {
            Element::Fire => p += 0.1,
            Element::Water => p -= 0.1,
            _ => {}
        }
    }
    rnjesus::binom(10, p)
}

fn get_climate(tile_elements: &TileElements) -> u8 {
    // fire vs air
    let mut p = 0.5;
    for element in tile_elements.elements.iter() {
        match element {
            Element::Fire => p += 0.1,
            Element::Air => p -= 0.1,
            _ => {}
        }
    }
    rnjesus::binom(10, p)
}

fn get_humidity(tile_elements: &TileElements) -> u8 {
    // water vs earth
    let mut p = 0.5;
    for element in tile_elements.elements.iter() {
        match element {
            Element::Water => p += 0.1,
            Element::Earth => p -= 0.1,
            _ => {}
        }
    }
    rnjesus::binom(10, p)
}

fn get_sea_type(vulcanism: &u8, humidity: &u8) -> SeaType {
    if vulcanism >= &8 {
        return SeaType::LavaOcean;
    } else if vulcanism <= &3 && humidity >= &8 {
        return SeaType::Ocean;
    }
    SeaType::None
}

fn get_vegetation(climate: &u8, humidity: &u8) -> u8 {
    let c: f64 = *climate as f64 / 25.0;
    let h: f64 = *humidity as f64 / 25.0;
    let p: f64 = (1.5 * h) + c;

    rnjesus::binom(10, p)
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
