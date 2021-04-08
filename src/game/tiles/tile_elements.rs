use std::fmt;

#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Debug)]
pub enum Element {
    Air,
    Earth,
    Fire,
    Water,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct TileElements {
    pub elements: [Element; 3],
    pub elements_label: String,
}

impl TileElements {
    pub fn new(elements: [Element; 3]) -> Self {
        let elements_label = get_element_label(elements);
        TileElements {
            elements,
            elements_label,
        }
    }
}

fn get_element_label(elements: [Element; 3]) -> String {
    "FFF".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_tile_elements() {
        let te = TileElements::new([Element::Air, Element::Earth, Element::Fire]);
        assert_eq!(te.elements[1], Element::Earth)
    }

    #[test]
    fn can_create_element_labels() {
        let e = [Element::Air, Element::Earth, Element::Fire];
        let label = get_element_label(e);
        assert_eq!(label, "FFF")
    }
}
