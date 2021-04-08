use rand;
use rand::prelude::*;

mod rules;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Element {
    Air,
    Earth,
    Fire,
    Water,
}

pub enum ThreeElements {
    Combination(Element, Element, Element),
}

fn element_from_index(index: usize) -> Element {
    [Element::Air, Element::Earth, Element::Fire, Element::Water][index]
}

fn random_element_x() -> Element {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, 3);
    element_from_index(index)
}

#[derive(Debug)]
pub struct Node {
    pub elements: [Element; 3],
    pub location: (i32, i32),
    vulcanism: u8,
}

impl Node {
    pub fn new(location: (i32, i32)) -> Self {
        let elements = Node::generate_elements();

        Node {
            elements,
            location,
            vulcanism: 0,
        }
    }

    fn generate_elements() -> [Element; 3] {
        [random_element_x(), random_element_x(), random_element_x()]
    }

    fn vulcanism(&self) -> u8 {
        match self.elements {
            [Element::Fire, Element::Fire, Element::Fire] => 8,
            [Element::Fire, Element::Fire, Element::Earth] => 8,
            [Element::Fire, _, _] => 1,
            [Element::Earth, _, _] => 1,
            [_, _, _] => 0,
        }
    }

    fn foo(&self) -> &str {
        let three_elements =
            ThreeElements::Combination(Element::Air, Element::Fire, Element::Water);
        match three_elements {
            ThreeElements::Combination(Element::Air, Element::Fire, Element::Water) => "got ya",
            _ => "no",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_three_elements() {
        let node = Node::new((0, 0));
        assert_eq!(node.elements.len(), 3);

        assert_eq!(node.foo(), "got ya");

        assert_eq!(node.vulcanism, 0);
        assert!(node.vulcanism() < 9);
    }
}
