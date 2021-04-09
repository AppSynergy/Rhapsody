use rand;
use rand::prelude::*;
use std::fmt;

#[derive(Debug)]
pub struct Tile {
    pub location: (i32, i32),
}

impl Tile {
    pub fn new(location: (i32, i32)) -> Self {
        Tile { location }
    }
}

#[derive(Debug)]
pub struct Level {
    width: i32,
    height: i32,
    board: Vec<Vec<i32>>,
    tiles: Vec<Tile>,
}

impl Level {
    pub fn new(width: i32, height: i32) -> Self {
        let mut board = Vec::new();
        for _ in 0..height {
            let row = vec![0; width as usize];
            board.push(row);
        }

        let mut tiles = Vec::new();

        // A series of 4-10 pseudo-random locations are chosen
        // (much less likely around the player's base and near to the edges of the overall map)
        // https://stackoverflow.com/questions/56326524/producing-a-linear-distribution-for-a-donut-shape
        // https://docs.rs/statrs/0.7.0/statrs/distribution/struct.Beta.html
        tiles.push(Tile::new(Level::generate_random_location(width, height)));
        tiles.push(Tile::new(Level::generate_random_location(width, height)));
        tiles.push(Tile::new(Level::generate_random_location(width, height)));
        tiles.push(Tile::new(Level::generate_random_location(width, height)));

        Level {
            width,
            height,
            board,
            tiles,
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
                for tile in &self.tiles {
                    if tile.location == (row as i32, col as i32) {
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
