pub struct TileProperties {
    pub topography: u8,
    pub vulcanism: u8,
    pub temperature: u8,
    pub humidity: u8,
    pub vegetation: u8,
}

impl TileProperties {
    pub fn new() -> Self {
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
        let tp = TileProperties::new();
        assert_eq!(tp.vegetation, 0)
    }
}
