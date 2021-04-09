use super::rnjesus;
use std::fmt;

#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Debug)]
pub enum Element {
    Air,
    Earth,
    Fire,
    Water,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Element {
    fn to_label(&self) -> &str {
        match self {
            Element::Air => "A",
            Element::Earth => "E",
            Element::Fire => "F",
            Element::Water => "W",
        }
    }
}

pub type ThreeElements = [Element; 3];

#[derive(Debug)]
pub struct TileElements {
    pub elements: ThreeElements,
    pub elements_label: String,
}

fn get_element_label(elements: ThreeElements) -> String {
    elements
        .iter()
        .map(|x| x.to_label().to_string())
        .collect::<String>()
}

impl TileElements {
    pub fn new(elements: ThreeElements) -> Self {
        let elements_label = get_element_label(elements);
        TileElements {
            elements,
            elements_label,
        }
    }

    pub fn spawn() -> Self {
        let elements: ThreeElements = [
            rnjesus::rand_element(),
            rnjesus::rand_element(),
            rnjesus::rand_element(),
        ];
        TileElements::new(elements)
    }

    pub fn propagate(tile_elements: &TileElements) -> Self {
        let elements;
        // Parents with three of the same elements will have a 50 percent chance of sharing the same elemental mix,
        if tile_elements.is_triple() {
            // TODO refactor
            let roll = rnjesus::d10();
            if roll <= 5 {
                elements = tile_elements.elements;
            // a 30 percent chance of one of the three being randomly re rolled,
            } else if roll <= 8 {
                elements = reroll_one(tile_elements.elements);
            // and a 20 percent chance for two to be re rolled.
            } else {
                elements = reroll_two(tile_elements.elements);
            }
            TileElements::new(elements)

        // Those with two of the same have 80/15/5 percent chance
        } else if tile_elements.is_double() {
            let roll = rnjesus::d20();
            if roll <= 16 {
                elements = tile_elements.elements;
                // a 30 percent chance of one of the three being randomly re rolled,
            } else if roll <= 19 {
                elements = reroll_one(tile_elements.elements);
            // and a 20 percent chance for two to be re rolled.
            } else {
                elements = reroll_two(tile_elements.elements);
            }
            TileElements::new(elements)

        // Those with all different have 90/5/5
        } else {
            let roll = rnjesus::d20();
            if roll <= 18 {
                elements = tile_elements.elements;
                // a 30 percent chance of one of the three being randomly re rolled,
            } else if roll <= 19 {
                elements = reroll_one(tile_elements.elements);
            // and a 20 percent chance for two to be re rolled.
            } else {
                elements = reroll_two(tile_elements.elements);
            }
            TileElements::new(elements)
        }
        // These are all just example numbers -
        // and I haven't included a chance for three re-rolls, but there should probably be one too!
    }

    fn is_triple(&self) -> bool {
        [Element::Air, Element::Earth, Element::Fire, Element::Water]
            .iter()
            .any(|element| self.elements.iter().filter(|&n| *n == *element).count() == 3)
    }

    fn is_double(&self) -> bool {
        [Element::Air, Element::Earth, Element::Fire, Element::Water]
            .iter()
            .any(|element| self.elements.iter().filter(|&n| *n == *element).count() == 2)
    }
}

fn reroll_two(elements: ThreeElements) -> ThreeElements {
    let which = rnjesus::rand_u8(0, 2);
    // TODO lazy
    let roll1 = rnjesus::rand_element();
    let roll2 = rnjesus::rand_element();
    let roll3 = rnjesus::rand_element();
    [
        if which == 0 { elements[0] } else { roll1 },
        if which == 1 { elements[1] } else { roll2 },
        if which == 2 { elements[2] } else { roll3 },
    ]
}

fn reroll_one(elements: ThreeElements) -> ThreeElements {
    let which = rnjesus::rand_u8(0, 2);
    let roll = rnjesus::rand_element();
    [
        if which == 0 { roll } else { elements[0] },
        if which == 1 { roll } else { elements[1] },
        if which == 2 { roll } else { elements[2] },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_tile_elements() {
        TileElements::spawn();
        let te = TileElements::new([Element::Air, Element::Earth, Element::Fire]);
        assert_eq!(te.elements[1], Element::Earth)
    }

    #[test]
    fn can_create_element_labels() {
        let e = [Element::Air, Element::Earth, Element::Water];
        let label = get_element_label(e);
        assert_eq!(label, "AEW")
    }
}
