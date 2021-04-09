use rand;
use rand::prelude::*;

use super::tiles::Element;

pub fn d20() -> u8 {
    rand_u8(1, 20)
}

pub fn rand_u8(x: u8, y: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(x, y + 1) as u8
}

pub fn rand_element() -> Element {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, 4);
    [Element::Air, Element::Earth, Element::Fire, Element::Water][index]
}
