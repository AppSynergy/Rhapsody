use rand;
use rand::prelude::*;

#[derive(PartialEq)]
#[derive(Debug)]
enum NodeElement {
    Air,
    Earth,
    Fire,
    Water
}

#[derive(Debug)]
struct Node {
    element: NodeElement,
    location: (i32, i32),
}

impl Node {
    fn new(element: NodeElement, location: (i32, i32)) -> Self {

        Node { element, location }
    }

}

struct Tile {
    vulcanism: i32,
    
}

#[derive(Debug)]
pub struct Level {
    width: i32,
    height: i32,
    board: Vec<Vec<i32>>,
    nodes: Vec<Node>,
}

impl Level {
    pub fn new(width: i32, height: i32) -> Self {
        let mut board = Vec::new();
        for _ in 0..height {
            let row = vec![0; width as usize];
            board.push(row);
        }


        let mut nodes = Vec::new();

        nodes.push(Node::new(NodeElement::Air, Level::generate_random_location(width, height)));
        nodes.push(Node::new(NodeElement::Earth, Level::generate_random_location(width, height)));
        nodes.push(Node::new(NodeElement::Fire, Level::generate_random_location(width, height)));
        nodes.push(Node::new(NodeElement::Water, Level::generate_random_location(width, height)));

        Level {
            width,
            height,
            board,
            nodes,
        }
    }

    fn generate_random_location(width: i32, height: i32) -> (i32, i32) {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0, width);
        let y = rng.gen_range(0, height);

        return (x, y);

    }
}


#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn can_create_levels() {
        let level = Level::new(200, 200);

        assert_eq!(level.nodes[0].element,  NodeElement::Air);

        assert_eq!(200, level.board.len());
        for node in level.nodes {
            assert!(node.location.0 < 200)
        }
    }
}
