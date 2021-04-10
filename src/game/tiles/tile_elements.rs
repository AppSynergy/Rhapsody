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
pub type AllElements = [Element; 4];

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

    // These tiles are spawned with three randomly chosen elemental values
    // (this can include duplicate rolls, eg two or three of the same element)
    pub fn spawn() -> Self {
        let elements: ThreeElements = [
            rnjesus::rand_element(),
            rnjesus::rand_element(),
            rnjesus::rand_element(),
        ];
        TileElements::new(elements)
    }

    pub fn propagate(tile_elements: &TileElements) -> Self {
        // Parents with three of the same elements will have a 50/30/20 percent chance
        if tile_elements.is_triple() {
            tile_elements.maybe_reroll(10, 16)
        // Those with two of the same have 80/15/5 percent chance
        } else if tile_elements.is_double() {
            tile_elements.maybe_reroll(16, 19)
        // Those with all different have 90/5/5
        } else {
            tile_elements.maybe_reroll(18, 19)
        }
        // These are all just example numbers -
        // and I haven't included a chance for three re-rolls, but there should probably be one too!
    }

    fn maybe_reroll(&self, t1: u8, t2: u8) -> TileElements {
        let elements;
        let roll = rnjesus::d20();
        if roll <= t1 {
            elements = self.elements;
        // a {t1} percent chance of one of the three being randomly re rolled,
        } else if roll <= t2 {
            elements = reroll_one(self.elements);
        // and a {t2} percent chance for two to be re rolled.
        } else {
            elements = reroll_two(self.elements);
        }
        TileElements::new(elements)
    }

    fn all_elements(&self) -> AllElements {
        [Element::Air, Element::Earth, Element::Fire, Element::Water]
    }

    fn is_triple(&self) -> bool {
        self.all_elements()
            .iter()
            .any(|element| self.has_element_n(element, 3))
    }

    fn is_double(&self) -> bool {
        self.all_elements()
            .iter()
            .any(|element| self.has_element_n(element, 2))
    }

    pub fn has_element(&self, element: &Element) -> bool {
        self.elements.iter().filter(|&i| *i == *element).count() >= 1
    }

    pub fn has_element_n(&self, element: &Element, n: u8) -> bool {
        self.elements.iter().filter(|&i| *i == *element).count() == n.into()
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
        assert_eq!(te.elements[1], Element::Earth);
    }

    #[test]
    fn can_create_element_labels() {
        let e = [Element::Air, Element::Earth, Element::Water];
        let label = get_element_label(e);
        assert_eq!(label, "AEW");
    }

    #[test]
    fn can_has_element() {
        let te = TileElements::new([Element::Air, Element::Air, Element::Fire]);
        assert!(!te.has_element_n(&Element::Air, 3));
        assert!(te.has_element_n(&Element::Air, 2));
        assert!(!te.has_element_n(&Element::Air, 1));
        assert!(!te.has_element_n(&Element::Fire, 2));
        assert!(te.has_element_n(&Element::Fire, 1));
        assert!(!te.has_element_n(&Element::Water, 1));
    }
}
