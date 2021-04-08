use rand;
use rand::prelude::*;

pub trait NodeTraits {
    fn children(&self) -> u8;
    fn distance(&self) -> u8;
}

enum CardPoint {
    North,
    East,
    South,
    West,
}

enum Arrangement {
    One(CardPoint),
    Two(CardPoint, CardPoint),
    Three(CardPoint, CardPoint, CardPoint),
}

fn rand_u8(x: u8, y: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(x, y) as u8
}

pub struct Node {
    pub elements_label: String,
    pub children: u8,
    pub arrangement: String,
    pub distance: u8,
    pub voids: u8,
    pub topography: u8,
    pub climate: u8,
    pub vulcanism: u8,
    pub humidity: u8,
    pub vegetation: u8,
}

impl Node {
    pub fn new(elements_label: &str) -> Self {
        Node {
            elements_label: elements_label.to_string(),
            children: 0,
            arrangement: "".to_string(),
            distance: 0,
            voids: 0,
            vegetation: 0,
            humidity: 0,
            vulcanism: 0,
            climate: 0,
            topography: 0,
        }
    }
}

impl NodeTraits for Node {
    fn children(&self) -> u8 {
        let label = &self.elements_label;
        match &label[..] {
            "AAA" => rand_u8(9, 12),

            "FFF" => rand_u8(12, 40),
            "AFF" => rand_u8(9, 20),
            "FFE" => rand_u8(9, 20),
            "FFW" => rand_u8(9, 16),

            "EEE" => 24,

            "WWW" => 40,
            "AWW" => 16,
            "EWW" => 16,
            "FWW" => 16,

            _ => 9,
        }
    }

    fn distance(&self) -> u8 {
        let label = &self.elements_label;
        match &label[..] {
            "AAA" => rand_u8(2, 4),
            "AAE" => rand_u8(2, 3),
            "AAF" => rand_u8(2, 3),
            "AAW" => rand_u8(2, 3),

            "FFF" => rand_u8(1, 3),
            "AFF" => rand_u8(0, 2),
            "FFE" => rand_u8(0, 2),
            "FFW" => rand_u8(0, 2),

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_nodes() {
        let mut node = Node::new("WWW");
        node.children = node.children();
        node.distance = node.distance();
        assert_eq!(40, node.children);
        assert_eq!(3, node.distance);
    }
}
