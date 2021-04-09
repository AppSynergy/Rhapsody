use rand;
use rand::prelude::*;

pub fn rand_u8(x: u8, y: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(x, y) as u8
}
