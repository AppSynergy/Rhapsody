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
        let e = [Element::Air, Element::Earth, Element::Water];
        let label = get_element_label(e);
        assert_eq!(label, "AEW")
    }
}
