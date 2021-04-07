use rand;
use rand::prelude::*;

#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Element {
    Air,
    Earth,
    Fire,
    Water
}

fn element_from_index(index: usize) -> Element {
    [Element::Air, Element::Earth, Element::Fire, Element::Water][index]
}

fn random_element() -> Element {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, 3);
    element_from_index(index)
}

#[derive(Debug)]
pub struct Node {
    pub elements: [Element; 3],
    pub location: (i32, i32),
}

impl Node {
    pub fn new(location: (i32, i32)) -> Self {
        let elements = Node::generate_elements();

        Node { elements, location }
    }

    pub fn generate_elements() -> [Element; 3] {
        [random_element(), random_element(), random_element()]
    }

}

#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn can_create_three_elements() {
        let node = Node::new((0, 0));

        assert_eq!(node.elements.len(), 3)

    }
}
