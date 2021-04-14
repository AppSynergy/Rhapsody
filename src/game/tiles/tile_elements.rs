use super::*;

use std::fmt;
use std::iter::FromIterator;

pub type ThreeElements = [Element; 3];

#[derive(Debug)]
pub struct TileElements {
    pub elements: ThreeElements,
    pub elements_label: String,
}

impl fmt::Display for TileElements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out: &String = &self
            .elements
            .iter()
            .map(|x| format!("{} ", x.to_icon()).to_string())
            .collect::<String>();
        write!(f, "{}", out)
    }
}

impl TileElements {
    pub fn new(elements: ThreeElements) -> Self {
        let elements_label = TileElements::get_element_label(elements);
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

    fn get_element_label(elements: ThreeElements) -> String {
        let mut labels: Vec<&str> = elements.iter().map(|x| x.to_label()).collect();
        labels.sort_by(|a, b| a.cmp(b));

        String::from_iter(labels)
    }

    fn reroll_two(&self) -> ThreeElements {
        let elements: ThreeElements = self.elements;
        let which = rnjesus::rand_u8(0, 2);
        let roll1 = rnjesus::rand_element();
        let roll2 = rnjesus::rand_element();
        let roll3 = if which == 0 { roll1 } else { roll2 };
        [
            if which == 0 { elements[0] } else { roll1 },
            if which == 1 { elements[1] } else { roll2 },
            if which == 2 { elements[2] } else { roll3 },
        ]
    }

    fn reroll_one(&self) -> ThreeElements {
        let elements: ThreeElements = self.elements;
        let which = rnjesus::rand_u8(0, 2);
        let roll = rnjesus::rand_element();
        [
            if which == 0 { roll } else { elements[0] },
            if which == 1 { roll } else { elements[1] },
            if which == 2 { roll } else { elements[2] },
        ]
    }

    fn maybe_reroll(&self, t1: u8, t2: u8) -> TileElements {
        let elements;
        let roll = rnjesus::d20();
        if roll <= t1 {
            elements = self.elements;
        // a {t1} percent chance of one of the three being randomly re rolled,
        } else if roll <= t2 {
            elements = self.reroll_one();
        // and a {t2} percent chance for two to be re rolled.
        } else {
            elements = self.reroll_two();
        }
        TileElements::new(elements)
    }

    fn all_elements(&self) -> std::slice::Iter<Element> {
        [Element::Air, Element::Earth, Element::Fire, Element::Water].iter()
    }

    // Elements Logic API
    pub fn is_triple(&self) -> bool {
        self.all_elements()
            .any(|element| self.is_triple_of(element))
    }

    pub fn is_triple_of(&self, element: &Element) -> bool {
        self.has_n(element, 3)
    }

    pub fn is_double(&self) -> bool {
        self.all_elements()
            .any(|element| self.is_double_of(element))
    }

    pub fn is_double_of(&self, element: &Element) -> bool {
        self.has_n(element, 2)
    }

    pub fn is_single(&self) -> bool {
        self.all_elements().all(|element| self.has_n(element, 1))
    }

    pub fn _has_any(&self, element: &Element) -> bool {
        self.elements.iter().filter(|&i| *i == *element).count() >= 1
    }

    pub fn has_n(&self, element: &Element, n: u8) -> bool {
        self.elements.iter().filter(|&i| *i == *element).count() == n.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new_tile_elements() {
        let te = TileElements::new([Element::Air, Element::Earth, Element::Fire]);
        assert_eq!(te.elements[0], Element::Air);
        assert_eq!(te.elements[1], Element::Earth);
        assert_eq!(te.elements[2], Element::Fire);
    }

    #[test]
    fn can_spawn_tile_elements() {
        TileElements::spawn();
    }

    #[test]
    fn can_create_element_labels() {
        let e = [Element::Water, Element::Air, Element::Earth];
        let label = TileElements::get_element_label(e);
        assert_eq!(label, "AEW");
    }

    #[test]
    fn can_reroll() {
        let te = TileElements::new([Element::Air, Element::Earth, Element::Fire]);
        te.reroll_one();
        te.reroll_two();
    }

    #[test]
    fn can_has_doubles() {
        let te = TileElements::new([Element::Air, Element::Water, Element::Water]);
        assert!(te.is_double());
        assert!(te.is_double_of(&Element::Water));
        assert!(!te.is_triple());
        assert!(!te.is_single());
    }

    #[test]
    fn can_has_triples() {
        let te = TileElements::new([Element::Fire, Element::Fire, Element::Fire]);
        assert!(te.is_triple_of(&Element::Fire));
        assert!(te.is_triple());
        assert!(!te.is_double());
        assert!(!te.is_single());
    }

    #[test]
    fn can_has() {
        let te = TileElements::new([Element::Air, Element::Air, Element::Fire]);
        assert!(!te.has_n(&Element::Air, 3));
        assert!(te.has_n(&Element::Air, 2));
        assert!(!te.has_n(&Element::Air, 1));
        assert!(!te.has_n(&Element::Fire, 2));
        assert!(te.has_n(&Element::Fire, 1));
        assert!(!te.has_n(&Element::Water, 1));
    }
}
