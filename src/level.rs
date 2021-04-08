use crate::node::Node;
use rand;
use rand::prelude::*;
use std::fmt;

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

        nodes.push(Node::new(Level::generate_random_location(width, height)));
        nodes.push(Node::new(Level::generate_random_location(width, height)));
        nodes.push(Node::new(Level::generate_random_location(width, height)));
        nodes.push(Node::new(Level::generate_random_location(width, height)));

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

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "╭")?;
        for _ in 0..self.height as usize {
            write!(f, "──")?;
        }
        write!(f, "╮\n")?;
        for row in 0..self.height as usize {
            write!(f, "│")?;
            for col in 0..self.width as usize {
                let mut filled = false;
                for node in &self.nodes {
                    if node.location == (row as i32, col as i32) {
                        write!(f, "N ")?;
                        filled = true;
                        break;
                    }
                }
                if !filled {
                    write!(f, "  ")?;
                }
            }
            write!(f, "│\n")?;
        }
        write!(f, "╰")?;
        for _ in 0..self.height as usize {
            write!(f, "──")?;
        }
        write!(f, "╯\n")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_levels() {
        let level = Level::new(200, 200);

        assert_eq!(200, level.board.len());
    }
}
