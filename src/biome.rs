pub struct Biome {
    temperature: i8,
    humidity: i8,
}

impl Biome {
    pub fn new() -> Self {
        let temperature = 0;
        let humidity = 0;

        Biome {
            temperature,
            humidity
        }
    }
}



#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn can_create_biomes() {
        let b = Biome::new();
        assert_eq!(0, b.temperature);
        assert_eq!(0, b.humidity);
    }
}
